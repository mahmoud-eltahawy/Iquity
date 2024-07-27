use crate::{message_notify, Paths};

use super::{Content, SLIDES_SPLITTER};

use config::{EmittedConfig, EmittedMarkdown, GlobalConfig, CONFIG_EVENT, CONTENT_EVENT};
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{
    event::ModifyKind, Config, Event, EventKind::Modify, RecommendedWatcher, RecursiveMode, Watcher,
};

use tauri::{AppHandle, Emitter, Manager};

use std::path::Path;

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

pub async fn watch_markdown(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let (mut watcher, mut rx) = watcher()?;
    let path = app.state::<Paths>().markdown.clone();
    if !path.exists() {
        return Ok(());
    }
    let parent = path.parent().unwrap();
    watcher.watch(parent, RecursiveMode::NonRecursive)?;

    let content = app.state::<Content>();

    loop {
        let Some(Ok(ev)) = rx.next().await else {
            continue;
        };
        let Modify(ModifyKind::Data(_)) = ev.kind else {
            continue;
        };
        let slides = read_markdown(&path).await?;
        let mut content_slides = content.slides.lock().unwrap();
        *content_slides = slides;
        let mut index = content.index.lock().unwrap();
        if *index > content_slides.len() - 1 {
            *index = content_slides.len() - 1;
        };
        emit_markdown(
            &app,
            *index,
            content_slides.len(),
            content_slides.get(*index).unwrap_or(&String::new()),
        );
    }
}

pub async fn watch_config(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let (mut watcher, mut rx) = watcher()?;
    let path = &app.state::<Paths>().config;
    let watch_path = path.parent().unwrap();

    watcher.watch(watch_path, RecursiveMode::NonRecursive)?;

    loop {
        let Some(Ok(ev)) = rx.next().await else {
            continue;
        };
        let Modify(ModifyKind::Data(_)) = ev.kind else {
            continue;
        };

        let global_config = match GlobalConfig::get(&path).await {
            Ok(gc) => gc,
            Err(err) => {
                message_notify(&app, "Config File Error".to_string(), err.to_string());
                continue;
            }
        };
        let lch = global_config.live_config_reload;

        let keys_help = markdown_compile(&global_config.keys.to_string());
        let emitted_config = EmittedConfig::new(global_config, keys_help);
        emit_config(&app, emitted_config);
        if !lch {
            break;
        }
    }
    Ok(())
}
use markdown::{self, CompileOptions, Options, ParseOptions};
use rayon::prelude::*;

pub async fn read_markdown<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let text = tokio::fs::read_to_string(path).await?;
    let slides = text
        .split(SLIDES_SPLITTER)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(markdown_compile)
        .collect();
    Ok(slides)
}

pub fn markdown_compile(source: &str) -> String {
    let compile = CompileOptions {
        allow_dangerous_html: true,
        allow_dangerous_protocol: true,
        ..CompileOptions::default()
    };
    let parse = ParseOptions::gfm();
    let options = Options { compile, parse };
    markdown::to_html_with_options(source, &options).unwrap()
}

pub fn emit_markdown(app: &AppHandle, index: usize, len: usize, slide: &String) {
    let output = EmittedMarkdown::new(index + 1, len, slide);
    app.emit(CONTENT_EVENT, output).unwrap();
}

pub fn emit_config(app: &AppHandle, config: EmittedConfig) {
    app.emit(CONFIG_EVENT, config).unwrap();
}
