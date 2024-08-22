mod components;
mod local_config;
mod utils;

use components::{help::help, markdown_preview::Markdown};
use leptos::{
    html::{self},
    prelude::*,
};
use local_config::Config;
use styling::Style;
use utils::{config_init, key_bindings, listen_to_config, listen_to_markdown, md_init};

use crate::components::markdown_preview::markdown_preview;

pub fn app() -> impl IntoView {
    let conf = Config::default();
    config_init(conf.clone());
    listen_to_config(conf.clone());

    let markdown = Markdown::default();
    listen_to_markdown(markdown);
    md_init();
    provide_context(markdown);

    let style = move || {
        Style::default()
            .fontsize()
            .px(conf.font_size.get())
            .margin()
            .px(5)
            .padding()
            .px(5)
            .to_string()
    };

    let keys_help = conf.keys_help;
    let port = conf.port.clone();
    key_bindings(conf);

    html::main()
        .style(style)
        .child((markdown_preview(port), help(keys_help), progress_bar()))
}

fn progress_bar() -> impl IntoView {
    let markdown = use_context::<Markdown>().unwrap();
    let max = move || markdown.len.get();
    let value = move || markdown.current.get();
    let style = Style::default()
        .accent_color()
        .dark_green()
        .position()
        .fixed()
        .bottom()
        .px(0)
        .height()
        .px(4)
        .width()
        .percent(100)
        .to_string();
    view! {
        <progress
            style=style
            value=value
            max=max
        />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(app);
}
