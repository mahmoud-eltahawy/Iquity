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

use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};
use tauri::{AppHandle, Emitter, Manager};

use markdown::{self, CompileOptions, Options, ParseOptions};
use rayon::prelude::*;
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

#[derive(Debug)]
struct CodeBlock {
    begin: usize,
    end: usize,
    content: String,
    lang: String,
}

pub fn code_syntax_highlight(source: &str) -> String {
    let mut source = source.chars().collect::<Vec<_>>();
    let codes = extract_code_blocks(&source);
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];
    for code in codes {
        println!("{:#?}", code);
        let syntax = ps.find_syntax_by_extension(&code.lang).unwrap();
        let html = highlighted_html_for_string(&code.content, &ps, syntax, theme).unwrap();
        source.splice(code.begin..(code.end + 3), html.chars().collect::<Vec<_>>());
    }

    String::from_iter(source)
}

fn extract_code_blocks(source: &[char]) -> Vec<CodeBlock> {
    let mut all_blocks = Vec::<CodeBlock>::new();
    let mut code_block = Vec::<char>::new();
    let mut code_lang = Vec::<char>::new();
    let mut code_began = None::<usize>;
    let mut lang_done = false;
    for (i, three_chars) in source.windows(3).enumerate() {
        let three_chars_are_seperator = three_chars.iter().all(|x| *x == '`');
        match code_began {
            Some(begin) => {
                if three_chars_are_seperator {
                    let block = CodeBlock {
                        begin,
                        end: i,
                        content: String::from_iter(code_block.iter()),
                        lang: String::from_iter(code_lang.iter()).trim().to_string(),
                    };
                    all_blocks.push(block);
                    code_block.clear();
                    code_lang.clear();
                    code_began = None;
                    lang_done = false;
                }
                if i.checked_sub(begin).is_some_and(|i| i >= three_chars.len()) {
                    let c = three_chars[0];
                    if lang_done {
                        code_block.push(c);
                    } else {
                        code_lang.push(c);
                        lang_done = c == '\n';
                    }
                }
            }
            None => {
                if three_chars_are_seperator {
                    code_began = Some(i);
                }
            }
        }
    }
    all_blocks
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
