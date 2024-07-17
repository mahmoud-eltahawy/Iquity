pub mod components;
pub mod contexts;
pub mod tauri;

use components::container::style_container;
use futures::StreamExt;
use leptos::prelude::*;
use leptos::spawn::spawn_local;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;
use tauri::notify_preview;

use crate::components::container::container;
use crate::components::editor::editor;
use crate::components::markdown_preview::markdown_preview;
use crate::contexts::config::config_provider;
use crate::contexts::markdown::{markdown_provider, Markdown};
use leptos::html::div;

pub fn editor_view() -> impl IntoView {
    let markdown = markdown_provider();
    provide_context(markdown);

    Effect::new(move |_| {
        let content = markdown.get().text;
        spawn_local(async move {
            notify_preview(content).await;
        });
    });

    style_container(
    div()
        .class("w-[calc(100vw-2.5rem)] flex flex-1 flex-row justify-center space-x-8 items-center h-[calc(100vh-8.5rem)]")
        .child(
            container(editor()),
        ))
}

pub fn preview_view() -> impl IntoView {
    use tauri_sys::event::listen;
    let markdown = RwSignal::new(Markdown::new());

    spawn_local(async move {
        let events = listen::<String>("content").await.unwrap();
        let (mut events, _) = futures::stream::abortable(events);

        while let Some(event) = events.next().await {
            markdown.update(|markdown| {
                markdown.text = event.payload;
            });
        }
    });
    provide_context(markdown);
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
