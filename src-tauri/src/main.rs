use axum::Router;
use config::{GlobalConfig, InitConfig};
use portpicker::pick_unused_port;
use tauri::{generate_context, App, AppHandle, Manager, State};
use tauri_plugin_notification::NotificationExt;
use utils::{emit_markdown, markdown_compile, read_markdown};

use std::{
    io::{stdout, Write},
    net::SocketAddr,
    path::PathBuf,
    str::FromStr,
    sync::Mutex,
};
use tauri_plugin_cli::CliExt;
use tower_http::services::ServeDir;

mod utils;

const SLIDES_SPLITTER: &str = "---";

const HELP_MESSAGE: &[u8] = r#"
    Welcom to iquity 
        the markdown compiler

    you called iquity without a markdown path
    
        you should call the program with the path to
    the target md file then the program will hot reload
    the content of the file every time you change 
    something in it. 


    EXAMPLE

    iquity ./README.md


    PREVIEW WINDOW KEYS    

    p => print to pdf

    j => next theme

    k => previous theme

    L => next slide

    H => previous slide

    = or + => increase font size    

    - or _ => decrease font size    

    ? or / => show this help message    

    esc => to hide this message    
"#
.as_bytes();

struct Content {
    slides: Mutex<Vec<String>>,
    index: Mutex<usize>,
}

impl Default for Content {
    fn default() -> Self {
        Self {
            slides: Mutex::new(Vec::new()),
            index: Mutex::new(0),
        }
    }
}

struct Paths {
    markdown: PathBuf,
    markdown_parent: PathBuf,
    config: PathBuf,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_cli::init())
        .manage(Content::default())
        .invoke_handler(tauri::generate_handler![
            conf_init, md_init, next_slide, prev_slide, notify,
        ])
        .setup(setup)
        .run(generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let matches = app.cli().matches().unwrap();
    let port = pick_unused_port().unwrap();
    let Some(markdown_path) = matches
        .args
        .get("path")
        .and_then(|x| x.value.as_str().and_then(|x| PathBuf::from_str(x).ok()))
    else {
        stdout().write_all(HELP_MESSAGE).unwrap();
        std::process::exit(0x0100);
    };
    let markdown_parent = PathBuf::from(markdown_path.clone().parent().unwrap());
    let paths = Paths {
        markdown: markdown_path,
        markdown_parent,
        config: GlobalConfig::config_path().unwrap(),
    };
    serve(paths.markdown_parent.clone(), port);
    app.manage(paths);
    let markdown_handle = app.app_handle().to_owned();
    let config_handle = markdown_handle.clone();
    app.manage(port);
    tokio::task::spawn(async move {
        if let Err(err) = utils::watch_markdown(markdown_handle).await {
            eprintln!("Watching error : {:#?}", err);
        };
    });
    tokio::task::spawn(async move {
        if let Err(err) = utils::watch_config(config_handle, port).await {
            eprintln!("Watching error : {:#?}", err);
        };
    });

    Ok(())
}

fn serve(path: PathBuf, port: u16) {
    tokio::task::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .expect("failed to bind address");
        let app = Router::new().nest_service("/", ServeDir::new(path));
        axum::serve(listener, app)
            .await
            .expect("failded to serve content");
    });
}

#[tauri::command]
async fn md_init(
    app: AppHandle,
    content: State<'_, Content>,
    path: State<'_, Paths>,
) -> Result<(), String> {
    let markdown_path = path.inner().markdown.clone();
    let slides = read_markdown(&markdown_path)
        .await
        .map_err(|x| x.to_string())?;
    let mut content_slides = content.slides.lock().unwrap();
    emit_markdown(&app, 0, slides.len(), &slides[0]);
    *content_slides = slides;
    Ok(())
}

#[tauri::command]
async fn conf_init(
    app: AppHandle,
    paths: State<'_, Paths>,
    port: State<'_, u16>,
) -> Result<InitConfig, String> {
    let config_path = paths.inner().config.clone();
    let conf = match GlobalConfig::get(&config_path).await {
        Ok(conf) => conf,
        Err(err) => {
            message_notify(&app, "config init error".to_string(), err.to_string());
            GlobalConfig::default()
        }
    };

    let keys_help = markdown_compile(conf.keys.to_string());

    Ok(InitConfig {
        conf,
        keys_help,
        port: *port.inner(),
    })
}

#[tauri::command]
fn notify(app: AppHandle, title: String, message: String) {
    message_notify(&app, title, message);
}
pub fn message_notify(app: &AppHandle, title: String, message: String) {
    app.notification()
        .builder()
        .title(title)
        .body(message)
        .show()
        .unwrap_or_default();
}

#[tauri::command]
fn next_slide(app: AppHandle, content: State<'_, Content>) {
    let slides = content.slides.lock().unwrap();
    let mut index = content.index.lock().unwrap();
    let slide = if *index < slides.len() - 1 {
        *index += 1;
        slides.get(*index).unwrap()
    } else {
        slides.last().unwrap()
    };
    emit_markdown(&app, *index, slides.len(), slide);
}

#[tauri::command]
fn prev_slide(app: AppHandle, content: State<'_, Content>) {
    let slides = content.slides.lock().unwrap();
    let mut index = content.index.lock().unwrap();
    *index = index.checked_sub(1).unwrap_or(0);
    let slide = slides.get(*index).unwrap();
    emit_markdown(&app, *index, slides.len(), slide);
}
