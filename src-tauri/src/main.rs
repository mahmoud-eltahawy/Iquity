use std::{fs, path::PathBuf};

use error::UbiquityError;
use md::*;
use tauri::{generate_context, AppHandle, Emitter};

use rfd::FileDialog;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_file,
            save_file,
            open_file_dialog,
            notify_preview,
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
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

#[tauri::command]
fn notify_preview(app: AppHandle, content: String) {
    app.emit_to("preview", "content", content).unwrap()
}

fn read_from_fs(path: PathBuf) -> Result<String, UbiquityError> {
    Ok(fs::read_to_string(path)?)
}

fn save_to_fs(path: PathBuf, contents: String) -> Result<(), UbiquityError> {
    Ok(fs::write(path, contents)?)
}
