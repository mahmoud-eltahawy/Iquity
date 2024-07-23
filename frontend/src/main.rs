mod components;
mod local_config;
mod utils;

use std::collections::BTreeMap;

use components::help::help;
use config::EmittedMarkdown;
use leptos::{html, prelude::*};
use local_config::{Config, THEMES, THEMES_SIZE};
use utils::{
    config_init, key_bindings, listen_to_config, listen_to_markdown, notify, silent_invoke,
};

use crate::components::markdown_preview::markdown_preview;

#[derive(Clone, Copy, Debug)]
pub struct Markdown {
    cache: RwSignal<BTreeMap<usize, Box<String>>>,
    content: RwSignal<Box<String>>,
    current: RwSignal<usize>,
    len: RwSignal<usize>,
}

impl Markdown {
    pub fn cache_call(&self, current: usize) -> bool {
        match self.cache.with_untracked(|xs| xs.get(&current).cloned()) {
            Some(content) => {
                self.content.set(content);
                if self.current.get_untracked() != current {
                    self.current.set(current);
                }
                true
            }
            None => false,
        }
    }
    pub fn cache_set(&self, index: usize, content: Box<String>) {
        self.cache.update_untracked(|xs| xs.insert(index, content));
    }
    pub fn set(&self, EmittedMarkdown { current, len }: EmittedMarkdown, content: Box<String>) {
        self.content.set(content);
        if self.current.get_untracked() != current {
            self.current.set(current);
        }
        if self.len.get_untracked() != len {
            self.len.set(len);
        }
    }
}

impl Default for Markdown {
    fn default() -> Self {
        Markdown {
            cache: RwSignal::new(BTreeMap::new()),
            content: RwSignal::new(Box::new(String::default())),
            current: RwSignal::new(0),
            len: RwSignal::new(0),
        }
    }
}

pub fn app() -> impl IntoView {
    let conf = Config::default();
    listen_to_config(conf);
    config_init(conf);

    let markdown = Markdown::default();
    listen_to_markdown(markdown);
    silent_invoke("md_init");
    provide_context(markdown);

    let theme = move || THEMES[conf.theme_index.get() % THEMES_SIZE];
    let font_size = move || conf.font_size.get();

    Effect::new(move |_| {
        let theme = theme().to_string();
        if conf.theme_notification.get_untracked() {
            notify("iquity theme", theme);
        }
    });

    Effect::new(move |_| {
        let current = markdown.current.get();
        let len = markdown.len.get();
        if current != 0 && len != 0 && conf.slide_notification.get_untracked() {
            notify("iquity slide", format!("[ {} / {} ]", current, len));
        }
    });

    key_bindings(conf);

    html::main()
        .attr("data-theme", theme)
        .class(font_size)
        .child((markdown_preview(), help()))
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(app);
}
