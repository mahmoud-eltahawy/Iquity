mod components;
mod local_config;
mod utils;

use components::{help::help, markdown_preview::Markdown};
use leptos::{
    html::{self},
    prelude::*,
};
use local_config::{Config, THEMES, THEMES_SIZE};
use utils::{
    config_init, key_bindings, listen_to_config, listen_to_markdown, notify, silent_invoke,
};

use crate::components::markdown_preview::markdown_preview;

pub fn app() -> impl IntoView {
    let conf = Config::default();
    config_init(conf.clone());
    listen_to_config(conf.clone());

    let markdown = Markdown::default();
    listen_to_markdown(markdown);
    silent_invoke("md_init");
    provide_context(markdown);

    let theme = move || THEMES[conf.theme_index.get() % THEMES_SIZE];
    let font_size = move || conf.font_size.get();

    Effect::new({
        let theme_notification = conf.theme_notification.clone();
        move |_| {
            let theme = theme().to_string();
            if *theme_notification.borrow() {
                notify("iquity theme", theme);
            }
        }
    });

    let keys_help = conf.keys_help;
    let port = conf.port.clone();
    key_bindings(conf);

    html::main()
        .attr("data-theme", theme)
        .class(font_size)
        .child((
            markdown_preview(port),
            help(keys_help),
            progress_bar(markdown),
        ))
}

fn progress_bar(markdown: Markdown) -> impl IntoView {
    let max = move || markdown.len.get();
    let value = move || markdown.current.get();
    view! {
        <progress
            class="progress progress-success fixed bottom-0 h-1 w-full"
            value=value
            max=max
        />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(app);
}
