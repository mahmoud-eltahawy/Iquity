use config::InitConfig;
use local_context::BackendContext;
use tauri::{generate_context, App, AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;
use utils::{emit_markdown, markdown_compile};

use std::{
    io::{stdout, Write},
    path::PathBuf,
    str::FromStr,
    sync::LazyLock,
};
use tauri_plugin_cli::CliExt;

mod local_context;
mod utils;

const SLIDES_SPLITTER: &str = "\n---";
static SLIDES_SPLITTER_AS_MD: LazyLock<String> =
    LazyLock::new(|| markdown_compile(SLIDES_SPLITTER.to_string()));

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

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_cli::init())
        .invoke_handler(tauri::generate_handler![
            conf_init,
            md_init,
            next_slide,
            prev_slide,
            notify,
            export_html,
            join_slides,
        ])
        .setup(setup)
        .run(generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let matches = app.cli().matches()?;
    let Some(markdown_path) = matches
        .args
        .get("path")
        .and_then(|x| x.value.as_str().and_then(|x| PathBuf::from_str(x).ok()))
    else {
        stdout().write_all(HELP_MESSAGE)?;
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
fn md_init(app: AppHandle) {
    let context = app.state::<BackendContext>();
    let slides = context.slides.lock().unwrap();
    emit_markdown(&app, 0, slides.len(), &slides[0]);
}

#[tauri::command]
fn join_slides(app: AppHandle) {
    let context = app.state::<BackendContext>();
    let slides = context.slides.lock().unwrap().join(&SLIDES_SPLITTER_AS_MD);
    emit_markdown(&app, 0, 1, &slides);
}

#[tauri::command]
fn conf_init(app: AppHandle) -> InitConfig {
    let context = app.state::<BackendContext>();
    let conf = context.config.clone();
    let keys_help = markdown_compile(conf.keys.to_string());

    InitConfig {
        conf,
        keys_help,
        port: context.port,
    }
}

#[tauri::command]
fn notify(app: AppHandle, title: String, message: String) {
    message_notify(&app, &title, &message);
}
pub fn message_notify(app: &AppHandle, title: &str, message: &str) {
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

#[tauri::command]
async fn export_html(app: AppHandle, html: String) {
    let context = app.state::<BackendContext>();
    let mut path = context.slides_home_path.clone();
    path.push("index.html");
    match tokio::fs::write(&path, html).await {
        Ok(_) => message_notify(
            &app,
            "Html Exporting",
            &format!("html is exported to file at {}", path.to_str().unwrap()),
        ),
        Err(err) => {
            message_notify(&app, "Html Exporting Problem", &err.to_string());
        }
    };
}
