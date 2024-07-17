use crate::contexts::markdown::Markdown;
use error::UbiquityError;
use md::*;
use tauri_sys::core::invoke;

pub async fn save_markdown_to_fs(markdown: Markdown) -> Result<String, UbiquityError> {
    let contents = markdown.text.to_string();
    let path = markdown.key.map(|key| key.to_string());
    let markdown: &MarkdownFile = &MarkdownFile { path, contents };
    let save_file: Result<String, String> = invoke("save_file", markdown).await;
    match save_file {
        Ok(path) => Ok(path),
        Err(tauri_error) => Err(UbiquityError::from(tauri_error)),
    }
}

pub async fn read_markdown_from_fs(path: String) -> Result<String, UbiquityError> {
    let markdown_file = &MarkdownPath { path };
    let read_file: Result<String, String> = invoke("read_file", markdown_file).await;
    match read_file {
        Ok(md) => Ok(md),
        Err(tauri_error) => Err(UbiquityError::from(tauri_error)),
    }
}

pub async fn notify_preview(content: String) -> Result<(), UbiquityError> {
    let markdown_content = &MarkdownContent { content };
    let read_file: Result<(), String> = invoke("notify_preview", markdown_content).await;
    match read_file {
        Ok(_) => Ok(()),
        Err(tauri_error) => Err(UbiquityError::from(tauri_error)),
    }
}

pub async fn create_new_markdown_file() -> Result<String, UbiquityError> {
    let contents = String::from("");
    let path = None;
    let markdown: &MarkdownFile = &MarkdownFile { path, contents };
    let save_file: Result<String, String> = invoke("save_file", markdown).await;
    match save_file {
        Ok(path) => Ok(path),
        Err(tauri_error) => Err(UbiquityError::from(tauri_error)),
    }
}
