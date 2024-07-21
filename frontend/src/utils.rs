use config::{EmittedMarkdown, CONTENT_EVENT};
use futures::StreamExt;
use tauri_sys::{core::invoke, event::listen};

use leptos::spawn::spawn_local;

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

pub fn silent_invoke(action: &'static str) {
    #[derive(Serialize, Deserialize)]
    pub struct Empty {}
    spawn_local(async move {
        invoke::<()>(action, Empty {}).await;
    });
}

pub fn listen_to_content(markdown: Markdown) {
    listen_to(CONTENT_EVENT, move |output: EmittedMarkdown<String>| {
        markdown.set(output);
    });
}
