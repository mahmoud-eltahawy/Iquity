use config::{GlobalConfig, CONFIG_EVENT};
use tauri::{generate_context, AppHandle, Emitter, Manager, State};
use tauri_plugin_notification::NotificationExt;
use utils::{emit_content, read_markdown};

use std::{
    io::{stdout, Write},
    path::PathBuf,
    str::FromStr,
    sync::Mutex,
};
use tauri_plugin_cli::CliExt;

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
    config: PathBuf,
}

#[tokio::main]
async fn main() {
    let config_path = GlobalConfig::config_path().unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_cli::init())
        .manage(Content::default())
        .invoke_handler(tauri::generate_handler![
            conf_init, md_init, next_slide, prev_slide, notify,
        ])
        .setup(move |app| {
            let matches = app.cli().matches().unwrap();
            let Some(markdown_path) = matches
                .args
                .get("path")
                .and_then(|x| x.value.as_str().map(|x| x.to_string()))
            else {
                stdout().write_all(HELP_MESSAGE).unwrap();
                std::process::exit(0x0100);
            };
            let paths = Paths {
                markdown: PathBuf::from_str(&markdown_path).unwrap(),
                config: config_path,
            };
            app.manage(paths);

            let handle = app.app_handle().to_owned();
            tokio::task::spawn(async move {
                utils::watch(handle, markdown_path).await.unwrap();
            });
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn md_init(
    app: AppHandle,
    content: State<'_, Content>,
    path: State<'_, Paths>,
) -> Result<(), String> {
    let slides = read_markdown(&path.inner().markdown)
        .await
        .map_err(|x| x.to_string())?;
    let mut content_slides = content.slides.lock().unwrap();
    emit_content(&app, 0, slides.len(), &slides[0]);
    *content_slides = slides;
    Ok(())
}

#[tauri::command]
async fn conf_init(app: AppHandle, paths: State<'_, Paths>) -> Result<(), String> {
    let config = GlobalConfig::get(&paths.inner().config)
        .await
        .map_err(|x| x.to_string())?;
    app.emit(CONFIG_EVENT, config).map_err(|x| x.to_string())?;
    Ok(())
}

#[tauri::command]
fn notify(app: AppHandle, title: String, message: String) {
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
    emit_content(&app, *index, slides.len(), slide);
}

#[tauri::command]
fn prev_slide(app: AppHandle, content: State<'_, Content>) {
    let slides = content.slides.lock().unwrap();
    let mut index = content.index.lock().unwrap();
    *index = index.checked_sub(1).unwrap_or(0);
    let slide = slides.get(*index).unwrap();
    emit_content(&app, *index, slides.len(), slide);
}
