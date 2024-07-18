pub mod components;

use config::Config;
use futures::StreamExt;
use leptos::{prelude::*, spawn::spawn_local};
use serde::de::DeserializeOwned;
use tauri_sys::event::listen;

use crate::components::markdown_preview::markdown_preview;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Markdown(pub String);

pub fn app() -> impl IntoView {
    let markdown = RwSignal::new(Markdown::default());
    provide_context(RwSignal::new(Config::default()));
    provide_context(markdown);

    listen_to("content", move |payload| {
        markdown.update(|markdown| {
            markdown.0 = payload;
        });
    });
    markdown_preview()
}

fn listen_to<F, T>(event: &'static str, fun: F)
where
    F: Fn(T) + 'static,
    T: DeserializeOwned + 'static,
{
    spawn_local(async move {
        let events = listen::<T>(event).await.unwrap();
        let (mut events, _) = futures::stream::abortable(events);

        loop {
            if let Some(event) = events.next().await {
                fun(event.payload);
            } else {
                continue;
            }
        }
    });
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(app);
}
