use tauri::{generate_context, AppHandle, Emitter, Manager};

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tauri_plugin_cli::CliExt;

const HELP_MESSAGE: &str = r#"
    you should call the program with the path to the target md file

    EXAMPLE

    iquity ./README.md

    PREVIEW WINDOW KEYS    

    p => print to pdf

    j => next theme

    k => previous theme

    = or + => increase font size    

    - or _ => decrease font size    

    ? or / => show this help message    

    esc => to hide this message    
"#;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .invoke_handler(tauri::generate_handler![
            // read_file
        ])
        .setup(move |app| {
            let matches = app.cli().matches().unwrap();
            let Some(path) = matches
                .args
                .get("path")
                .and_then(|x| x.value.as_str().map(|x| x.to_string()))
            else {
                println!("{}", HELP_MESSAGE);
                std::process::exit(0x0100);
            };

            let handle = app.app_handle().to_owned();
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
        let content = read_file(&path).await?;
        app.emit("content", content)?;
    }
}

async fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn std::error::Error>> {
    Ok(tokio::fs::read_to_string(path).await?)
}
