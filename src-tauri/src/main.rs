use std::{fs, path::PathBuf};

use error::UbiquityError;
use md::*;
use tauri::{generate_context, AppHandle, Emitter, Manager};

use rfd::FileDialog;

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_file,
            save_file,
            open_file_dialog,
            notify_preview,
        ])
        .setup(move |app| {
            let handle = app.app_handle().clone();
            tokio::task::spawn(async move {
                let path = "../README.md";
                async_watch(handle, path).await.unwrap();
            });
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(
    app: AppHandle,
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(_) = rx.next().await {
        let content = async_read_file(path.as_ref().to_str().unwrap().to_string()).await?;
        app.emit_to("preview", "content", content).unwrap();
    }

    Ok(())
}
#[tauri::command]
fn save_file(path: Option<String>, contents: String) -> Result<String, UbiquityError> {
    match path {
        Some(path_key) if !path_key.eq(&DOCS_KEY) => {
            let path = PathBuf::from(path_key.clone());

            match save_to_fs(path, contents) {
                Ok(_) => Ok(path_key),
                Err(err) => Err(err),
            }
        }
        _ => {
            let mut dir = PathBuf::from("/");
            if let Some(docs_dir) = dirs::document_dir() {
                dir = docs_dir;
            }
            let file_dialog = FileDialog::new().set_directory(dir).save_file();

            match file_dialog {
                Some(file_handle) => {
                    fs::write(file_handle.clone(), contents)?;
                    Ok(file_handle.to_str().unwrap().to_string())
                }
                None => Err(UbiquityError::no_save_path_selected()),
            }
        }
    }
}

#[tauri::command]
fn open_file_dialog() -> Result<MarkdownFile, UbiquityError> {
    let mut dir = PathBuf::from("/");
    if let Some(docs_dir) = dirs::document_dir() {
        dir = docs_dir;
    }
    let file_dialog_res = FileDialog::new().set_directory(dir).pick_file();

    if let Some(file_handle) = file_dialog_res {
        let contents = read_from_fs(file_handle.clone())?;
        let path = Some(file_handle.to_str().unwrap().to_string());
        let markdown_file = MarkdownFile { path, contents };
        Ok(markdown_file)
    } else {
        Err(UbiquityError::no_file_selected())
    }
}

#[tauri::command]
fn read_file(path: String) -> Result<String, UbiquityError> {
    Ok(fs::read_to_string(PathBuf::from(path))?)
}

async fn async_read_file(path: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(tokio::fs::read_to_string(PathBuf::from(path)).await?)
}

#[tauri::command]
fn notify_preview(app: AppHandle, content: String) {
    app.emit_to("preview", "content", content).unwrap();
}

fn read_from_fs(path: PathBuf) -> Result<String, UbiquityError> {
    Ok(fs::read_to_string(path)?)
}

fn save_to_fs(path: PathBuf, contents: String) -> Result<(), UbiquityError> {
    Ok(fs::write(path, contents)?)
}
