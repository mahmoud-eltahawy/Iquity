pub mod components;
pub mod contexts;

use futures::StreamExt;
use leptos::{prelude::*, spawn::spawn_local};
use tauri_sys::event::listen;

use crate::components::markdown_preview::markdown_preview;
use crate::contexts::config::config_provider;
use crate::contexts::markdown::Markdown;

pub fn app() -> impl IntoView {
    let markdown = RwSignal::new(Markdown::new());
    provide_context(config_provider());
    provide_context(markdown);

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
    markdown_preview()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(app);
}
