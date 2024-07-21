mod components;
mod local_config;
mod utils;

use components::{help::help, notification::notification, which_slide::which_slide};
use config::EmittedMarkdown;
use leptos::{html, prelude::*};
use local_config::{Config, THEMES, THEMES_SIZE};
use utils::{key_bindings, listen_to_content, silent_invoke};

use crate::components::markdown_preview::markdown_preview;

#[derive(Clone, Copy, Debug)]
pub struct Markdown {
    content: RwSignal<String>,
    current: RwSignal<usize>,
    len: RwSignal<usize>,
}

impl From<EmittedMarkdown<String>> for Markdown {
    fn from(
        EmittedMarkdown {
            current,
            len,
            content,
        }: EmittedMarkdown<String>,
    ) -> Self {
        Self {
            content: RwSignal::new(content),
            current: RwSignal::new(current),
            len: RwSignal::new(len),
        }
    }
}

impl Markdown {
    pub fn set(
        &self,
        EmittedMarkdown {
            current,
            len,
            content,
        }: EmittedMarkdown<String>,
    ) {
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
            content: RwSignal::new(String::default()),
            current: RwSignal::new(0),
            len: RwSignal::new(0),
        }
    }
}

pub fn app() -> impl IntoView {
    let conf = Config::default();

    let markdown = Markdown::default();
    listen_to_content(markdown);
    silent_invoke("md_init");
    provide_context(markdown);

    let theme = move || THEMES[conf.theme_index.get() % THEMES_SIZE];
    let font_size = move || conf.font_size.get();

    key_bindings(conf);

    html::main()
        .attr("data-theme", theme)
        .class(font_size)
        .child((
            markdown_preview(),
            help(),
            which_slide(),
            notification(theme),
        ))
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(app);
}
