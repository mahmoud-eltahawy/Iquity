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
    let paths = app.state::<Paths>();
    let path = paths.markdown.clone();
    let parent = &paths.markdown_parent;
    if !path.exists() || !parent.exists() {
        return Ok(());
    }
    watcher.watch(parent.as_path(), RecursiveMode::NonRecursive)?;

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

pub async fn watch_config(app: AppHandle, port: u16) -> Result<(), Box<dyn std::error::Error>> {
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

        let keys_help = markdown_compile(global_config.keys.to_string());
        let emitted_config = EmittedConfig::new(global_config, keys_help, port);
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
        .map(code_syntax_highlight)
        .map(markdown_compile)
        .collect();
    Ok(slides)
}

pub fn code_syntax_highlight(source: &str) -> String {
    let mut source = source.chars().collect::<Vec<_>>();
    struct Code {
        begin: usize,
        end: usize,
        content: Vec<char>,
    }
    let mut codes = Vec::<Code>::new();
    let mut code = Vec::<char>::new();
    let mut code_began = None::<usize>;
    for (i, cs) in source.as_slice().windows(3).enumerate() {
        if cs.iter().all(|x| *x == '`') {
            if let Some(begin) = code_began {
                let c = Code {
                    begin,
                    end: i,
                    content: code.clone(),
                };
                codes.push(c);
                code.clear();
                code_began = None;
            } else {
                code_began = Some(i);
            }
        }
        if let Some(_) = code_began {
            code.push(*cs.first().unwrap());
        }
    }
    for mut code in codes {
        if code.content.len() > 3 {
            code.content = code.content[3..].to_vec();
        };
        let code_content = String::from_iter(code.content);
        let (lang, code_content) = match code_content.split_once('\n') {
            Some((lang, code)) => (lang.to_string(), code.to_string()),
            None => ("".to_string(), code_content),
        };

        println!(
            "\nlang : {}\ncode :\n{}\nbegin : {} , end : {}",
            lang, code_content, code.begin, code.end
        );
        source.splice(
            code.begin..code.end,
            "```HELLO CODE".chars().collect::<Vec<_>>(),
        );
    }

    String::from_iter(source)
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
