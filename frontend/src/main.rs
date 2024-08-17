mod components;
mod local_config;
mod style;
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
    use style::{
        style,
        CssAttribute::{FontSize, Margin, Padding},
        Length,
    };
    let conf = Config::default();
    config_init(conf.clone());
    listen_to_config(conf.clone());

    let markdown = Markdown::default();
    listen_to_markdown(markdown);
    silent_invoke("md_init");
    provide_context(markdown);

    let style = move || {
        style(vec![
            Margin(Length::Px(5)),
            Padding(Length::Px(5)),
            FontSize(Length::Px(conf.font_size.get())),
        ])
    };

    let keys_help = conf.keys_help;
    let port = conf.port.clone();
    key_bindings(conf);

    html::main()
        .style(style)
        .child((markdown_preview(port), help(keys_help), progress_bar()))
}

fn progress_bar() -> impl IntoView {
    use style::{
        style,
        Color::Hex,
        CssAttribute::*,
        CssPosition::Fixed,
        Length::{Percent, Px},
    };
    let markdown = use_context::<Markdown>().unwrap();
    let max = move || markdown.len.get();
    let value = move || markdown.current.get();
    let style = style(vec![
        Position(Fixed),
        BackgroundColor(Hex(0x4CAF50)),
        Bottom(Px(0)),
        Height(Px(4)),
        Width(Percent(100)),
    ]);
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
