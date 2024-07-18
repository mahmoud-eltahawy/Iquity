use std::path::PathBuf;

use tauri::{generate_context, AppHandle, Emitter, Manager};

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tauri_plugin_cli::CliExt;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .invoke_handler(tauri::generate_handler![
            // read_file
        ])
        .setup(move |app| {
            let path = app.cli().matches().unwrap();
            let path = path
                .args
                .get("path")
                .unwrap()
                .value
                .as_str()
                .unwrap()
                .to_string();

            let handle = app.app_handle().clone();
            tokio::task::spawn(async move {
                watch(handle, path).await.unwrap();
            });
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}

fn watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
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

async fn watch<P: AsRef<Path>>(app: AppHandle, path: P) -> Result<(), Box<dyn std::error::Error>> {
    let (mut watcher, mut rx) = watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    loop {
        if rx.next().await.is_none() {
            continue;
        }
        let content = read_file(path.as_ref().to_str().unwrap().to_string()).await?;
        app.emit_to("preview", "content", content)?;
    }
}

async fn read_file(path: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(tokio::fs::read_to_string(PathBuf::from(path)).await?)
}
