mod components;
mod local_config;
mod utils;

use components::{help::help, markdown_preview::Markdown};
use leptos::{
    html::{self},
    prelude::*,
};
use local_config::Config;
use utils::{config_init, key_bindings, listen_to_config, listen_to_markdown, silent_invoke};

use crate::components::markdown_preview::markdown_preview;

pub fn app() -> impl IntoView {
    let conf = Config::default();
    config_init(conf.clone());
    listen_to_config(conf.clone());

    let markdown = Markdown::default();
    listen_to_markdown(markdown);
    silent_invoke("md_init");
    provide_context(markdown);

    let font_size = move || format!("font-size : {}px;", conf.font_size.get());

    let keys_help = conf.keys_help;
    let port = conf.port.clone();
    key_bindings(conf);

    html::main()
        .style(font_size)
        .child((markdown_preview(port), help(keys_help), progress_bar()))
}

fn progress_bar() -> impl IntoView {
    let markdown = use_context::<Markdown>().unwrap();
    let max = move || markdown.len.get();
    let value = move || markdown.current.get();
    let style = "background-color: #4CAF50;position: fixed;bottom: 0;height: 4px;width: 100%;";
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
