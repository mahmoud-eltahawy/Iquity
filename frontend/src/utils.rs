use futures::StreamExt;
use tauri_sys::{core::invoke, event::listen};

use leptos::{reactive_graph::traits::Set, spawn::spawn_local};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::Markdown;

pub fn listen_to<F, T>(event: &'static str, fun: F)
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

#[derive(Serialize, Deserialize)]
pub struct Empty {}

pub fn init_markdown() -> Markdown {
    let markdown = Markdown::default();
    spawn_local(async move {
        markdown.set(invoke::<String>("md_init", Empty {}).await);
    });
    markdown
}

pub fn next_slide() {
    spawn_local(async move {
        invoke::<String>("next_slide", Empty {}).await;
    });
}

pub fn prev_slide() {
    spawn_local(async move {
        invoke::<String>("prev_slide", Empty {}).await;
    });
}
