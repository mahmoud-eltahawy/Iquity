pub mod components;
pub mod contexts;
pub mod tauri;

use futures::StreamExt;
use leptos::{prelude::*, spawn::spawn_local};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};
use tauri::notify_preview;
use tauri_sys::event::listen;

use crate::components::editor::editor;
use crate::components::markdown_preview::markdown_preview;
use crate::contexts::config::config_provider;
use crate::contexts::markdown::{markdown_provider, Markdown};

pub fn editor_view() -> impl IntoView {
    let markdown = markdown_provider();
    provide_context(markdown);

    Effect::new(move |_| {
        let content = markdown.get().text;
        spawn_local(async move {
            notify_preview(content).await;
        });
    });

    editor()
}

pub fn preview_view() -> impl IntoView {
    let markdown = RwSignal::new(Markdown::new());

    spawn_local(async move {
        let events = listen::<String>("content").await.unwrap();
        let (mut events, _) = futures::stream::abortable(events);

        while let Some(event) = events.next().await {
            markdown.update(|markdown| {
                markdown.text = event.payload;
            });
        }
        unreachable!()
    });
    provide_context(markdown);
    markdown_preview()
}

fn app() -> impl IntoView {
    provide_context(config_provider());

    view! {
        <Router>
            <FlatRoutes fallback=|| view!{"".to_string()}>
                <Route path=StaticSegment("editor") view=editor_view/>
                <Route path=StaticSegment("preview") view=preview_view/>
            </FlatRoutes>
        </Router>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(app);
}
