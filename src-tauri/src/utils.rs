use crate::{message_notify, BackendContext};

use super::SLIDES_SPLITTER;

use config::{EmittedConfig, EmittedMarkdown, GlobalConfig, CONFIG_EVENT, CONTENT_EVENT};
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{
    event::ModifyKind, Config, Event, EventKind::Modify, RecommendedWatcher, RecursiveMode, Watcher,
};

use tauri::{AppHandle, Emitter, Manager};

use markdown::{self, CompileOptions, Options, ParseOptions};
use rayon::prelude::*;
use std::path::Path;

mod code_syntax_highlight;
use code_syntax_highlight::code_syntax_highlight;

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
    let context = app.state::<BackendContext>();
    let path = context.slides_path.clone();
    let parent = &context.slides_home_path;
    watcher.watch(parent.as_path(), RecursiveMode::NonRecursive)?;

    loop {
        let Some(Ok(ev)) = rx.next().await else {
            continue;
        };
        let Modify(ModifyKind::Data(_)) = ev.kind else {
            continue;
        };
        let slides = read_markdown(&path).await?;
        let mut content_slides = context.slides.lock().unwrap();
        *content_slides = slides;
        let mut index = context.slide_index.lock().unwrap();
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

pub async fn watch_config(app: AppHandle, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let (mut watcher, mut rx) = watcher()?;
    let path = &app.state::<BackendContext>().config_path;
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
                message_notify(&app, "Config File Error", &err.to_string());
                continue;
            }
        };
        let lch = global_config.live_config_reload;

        let keys_help = markdown_compile(global_config.keys.to_string());
        let emitted_config = EmittedConfig::new(global_config, keys_help, port);
        emit_config(&app, emitted_config);
        if !lch {
            break;
        }
    }
    Ok(())
}

pub async fn read_markdown<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let text = tokio::fs::read_to_string(path).await?;
    let slides = text
        .split(SLIDES_SPLITTER)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(code_syntax_highlight)
        .map(markdown_compile)
        .collect();
    Ok(slides)
}

pub fn markdown_compile(source: String) -> String {
    let compile = CompileOptions {
        allow_dangerous_html: true,
        allow_dangerous_protocol: true,
        ..CompileOptions::default()
    };
    let parse = ParseOptions::gfm();
    let options = Options { compile, parse };
    markdown::to_html_with_options(&source, &options).unwrap()
}

pub fn emit_markdown(app: &AppHandle, index: usize, len: usize, slide: &String) {
    let output = EmittedMarkdown::new(index + 1, len, slide);
    app.emit(CONTENT_EVENT, output).unwrap();
}

pub fn emit_config(app: &AppHandle, config: EmittedConfig) {
    app.emit(CONFIG_EVENT, config).unwrap();
}
