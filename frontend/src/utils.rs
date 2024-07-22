use config::{EmittedMarkdown, CONTENT_EVENT};
use futures::StreamExt;
use gloo::utils::{document, window};
use tauri_sys::{core::invoke, event::listen};

use leptos::{ev, prelude::*, spawn::spawn_local};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlDialogElement;

use crate::components::help::HELP_ID;
use crate::{local_config::Config, Markdown};

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
    struct Empty {}
    spawn_local(async move {
        invoke::<()>(action, Empty {}).await;
    });
}

pub fn notify(title: &'static str, message: String) {
    #[derive(Serialize, Deserialize)]
    struct Content {
        title: &'static str,
        message: String,
    }
    spawn_local(async move {
        invoke::<()>("notify", Content { title, message }).await;
    });
}

pub fn listen_to_content(markdown: Markdown) {
    listen_to(CONTENT_EVENT, move |output: EmittedMarkdown<String>| {
        markdown.set(output);
    });
}

pub fn key_bindings(conf: Config) {
    window_event_listener(ev::keydown, move |ke: ev::KeyboardEvent| {
        let code = ke.code();

        if code.eq("KeyP") {
            window().print().unwrap_throw();
        }

        if code.eq("KeyJ") {
            conf.next_theme();
        }

        if code.eq("KeyK") {
            conf.prev_theme();
        }

        if code.eq("KeyL") {
            silent_invoke("next_slide");
        }

        if code.eq("KeyH") {
            silent_invoke("prev_slide");
        }

        if code.eq("Minus") {
            conf.decrease_font_size();
        }

        if code.eq("Equal") {
            conf.increase_font_size();
        }

        if code == "Slash" {
            let dialog: HtmlDialogElement = document()
                .get_element_by_id(HELP_ID)
                .unwrap()
                .dyn_into()
                .unwrap();
            dialog.show_modal().unwrap();
        }
    });
}
