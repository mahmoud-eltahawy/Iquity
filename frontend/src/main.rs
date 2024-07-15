pub mod components;
pub mod contexts;
pub mod tauri;

use leptos::{html::div, prelude::*};

use crate::components::dual_view::dual_view;
use crate::contexts::config::config_provider;
use crate::contexts::markdown::markdown_provider;

fn app() -> impl IntoView {
    let conf = config_provider();
    provide_context(conf);
    provide_context(markdown_provider());

    let theme = move || conf.get().theme;

    div()
        .attr("data-theme", theme)
        .attr("class", "flex flex-col justify-between max-w-[calc(100svw)] print:hidden min-h-screen")
        .child(div().class("h-[calc(100vh-4rem)] flex flex-col content-center align-center items-center justify-center")
            .child(dual_view())
        )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(app);
}
