use axum::Router;
use config::{GlobalConfig, InitConfig};
use tauri::{generate_context, App, AppHandle, Manager};
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

const SLIDES_SPLITTER: &str = "\n---";

const HELP_MESSAGE: &[u8] = r#"
    Welcom to iquity 
        the markdown previewer

    you called iquity without a markdown path
    
        you should call the program with the path to
    the target md file or a path to directory that contains index.md file
    then the program will hot reload the content of the file and the 
    directory content every time you change something in it. 
"#
.as_bytes();

struct BackendContext {
    markdown_path: PathBuf,
    markdown_parent_path: PathBuf,
    config_path: PathBuf,
    port: u16,
    slides: Mutex<Vec<String>>,
    slide_index: Mutex<usize>,
}

impl BackendContext {
    async fn new(path: PathBuf, port: u16) -> Result<Self, String> {
        let (markdown_path, markdown_parent_path) = if path.is_file() {
            (path.clone(), path.parent().unwrap().into())
        } else if path.is_dir() {
            let mut son = path.clone();
            son.push("index.md");
            if son.exists() {
                (son, path)
            } else {
                return Err("can not find index.md".to_string());
            }
        } else {
            return Err("provided path does not exist".to_string());
        };

        let slides = read_markdown(&markdown_path)
            .await
            .map_err(|x| x.to_string())?;
        Ok(BackendContext {
            markdown_path,
            markdown_parent_path,
            config_path: GlobalConfig::config_path().unwrap(),
            port,
            slides: Mutex::new(slides),
            slide_index: Mutex::new(0),
        })
    }

    fn serve_assets(&self) {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let app = Router::new().nest_service("/", ServeDir::new(&self.markdown_parent_path));
        tokio::task::spawn(async move {
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .expect("failed to bind address");
            axum::serve(listener, app)
                .await
                .expect("failded to serve content");
        });
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_cli::init())
        .invoke_handler(tauri::generate_handler![
            conf_init, md_init, next_slide, prev_slide, notify,
        ])
        .setup(setup)
        .run(generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let matches = app.cli().matches().unwrap();
    let Some(markdown_path) = matches
        .args
        .get("path")
        .and_then(|x| x.value.as_str().and_then(|x| PathBuf::from_str(x).ok()))
    else {
        stdout().write_all(HELP_MESSAGE).unwrap();
        std::process::exit(0x0100);
    };

    let app_handle_1 = app.app_handle().clone();
    tokio::task::spawn(async move {
        let port = portpicker::pick_unused_port().unwrap();
        let context = BackendContext::new(markdown_path, port).await.unwrap();
        context.serve_assets();
        app_handle_1.manage(context);
        let app_handle_2 = app_handle_1.clone();
        let app_handle_3 = app_handle_1.clone();

        tokio::task::spawn(async move {
            if let Err(err) = utils::watch_markdown(app_handle_2).await {
                eprintln!("Markdown Watching error : {:#?}", err);
            };
        });
        tokio::task::spawn(async move {
            if let Err(err) = utils::watch_config(app_handle_3, port).await {
                eprintln!("Config Watching error : {:#?}", err);
            };
        });
    });

    Ok(())
}

#[tauri::command]
async fn md_init(app: AppHandle) -> Result<(), String> {
    let context = app.state::<BackendContext>();
    let slides = context.slides.lock().unwrap();
    emit_markdown(&app, 0, slides.len(), &slides[0]);
    Ok(())
}

#[tauri::command]
async fn conf_init(app: AppHandle) -> Result<InitConfig, String> {
    let context = app.state::<BackendContext>();
    let config_path = context.inner().config_path.clone();
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
        port: context.port,
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
fn next_slide(app: AppHandle) {
    let context = app.state::<BackendContext>();
    let slides = context.slides.lock().unwrap();
    let mut index = context.slide_index.lock().unwrap();
    let slide = if *index < slides.len() - 1 {
        *index += 1;
        slides.get(*index).unwrap()
    } else {
        slides.last().unwrap()
    };
    emit_markdown(&app, *index, slides.len(), slide);
}

#[tauri::command]
fn prev_slide(app: AppHandle) {
    let context = app.state::<BackendContext>();
    let slides = context.slides.lock().unwrap();
    let mut index = context.slide_index.lock().unwrap();
    *index = index.checked_sub(1).unwrap_or(0);
    let slide = slides.get(*index).unwrap();
    emit_markdown(&app, *index, slides.len(), slide);
}
