pub mod components;
pub mod contexts;
pub mod tauri;

use components::container::style_container;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use crate::components::container::container;
use crate::components::editor::editor;
use crate::components::markdown_preview::markdown_preview;
use crate::contexts::config::config_provider;
use crate::contexts::markdown::markdown_provider;
use leptos::html::div;

pub fn editor_view() -> impl IntoView {
    provide_context(markdown_provider());
    style_container(
    div()
        .class("w-[calc(100vw-2.5rem)] flex flex-1 flex-row justify-center space-x-8 items-center h-[calc(100vh-8.5rem)]")
        .child(
            container(editor()),
        ))
}

pub fn preview_view() -> impl IntoView {
    provide_context(markdown_provider()); //TODO :it should recieve it not creating it
    style_container(
    div()
        .class("w-[calc(100vw-2.5rem)] flex flex-1 flex-row justify-center space-x-8 items-center h-[calc(100vh-8.5rem)]")
        .child(
            container(markdown_preview())
        ))
}

fn app() -> impl IntoView {
    let conf = config_provider();
    provide_context(conf);

    let theme = move || conf.get().theme;
    view! {
        <main
            data-theme=theme
            class="flex flex-col justify-between max-w-[calc(100svw)] print:hidden min-h-screen"
        >
        <Router>
            <Routes fallback=|| view!{"".to_string()}>
                <Route path=StaticSegment("editor") view=editor_view/>
                <Route path=StaticSegment("preview") view=preview_view/>
            </Routes>
        </Router>
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(app);
}
