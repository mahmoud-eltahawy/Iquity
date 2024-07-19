pub mod components;
pub mod local_config;

use futures::StreamExt;
use gloo::utils::window;
use leptos::{ev, prelude::*, spawn::spawn_local};
use local_config::Config;
use serde::de::DeserializeOwned;
use tauri_sys::event::listen;
use wasm_bindgen::UnwrapThrowExt;

use crate::components::markdown_preview::markdown_preview;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Markdown(pub String);

pub fn app() -> impl IntoView {
    let markdown = RwSignal::new(Markdown::default());
    let conf = Config::default();

    window_event_listener(ev::keydown, move |ke: ev::KeyboardEvent| {
        if ke.code().eq("KeyP") {
            window().print().unwrap_throw();
        }

        if ke.code().eq("KeyJ") {
            conf.next_theme();
        }

        if ke.code().eq("KeyK") {
            conf.prev_theme();
        }

        if ke.code().eq("Minus") {
            conf.decrease_font_size();
        }

        if ke.code().eq("Equal") {
            conf.increase_font_size();
        }
    });

    provide_context(conf);
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
