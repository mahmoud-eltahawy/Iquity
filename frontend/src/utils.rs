use config::{EmittedConfig, EmittedMarkdown, CONTENT_EVENT};
use config::{GlobalConfig, CONFIG_EVENT};
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
    F: Fn(T) -> bool + 'static,
    T: DeserializeOwned + 'static,
{
    spawn_local(async move {
        let events = listen::<T>(event).await.unwrap();
        let (mut events, _) = futures::stream::abortable(events);

        loop {
            if let Some(event) = events.next().await {
                if fun(event.payload) {
                    break;
                };
            } else {
                continue;
            }
        }
    });
}
#[derive(Serialize, Deserialize)]
struct Empty {}

pub fn silent_invoke(action: &'static str) {
    spawn_local(async move {
        invoke::<()>(action, Empty {}).await;
    });
}

pub fn config_init(conf: Config) {
    spawn_local(async move {
        conf.set(invoke::<GlobalConfig>("conf_init", Empty {}).await);
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

pub fn listen_to_markdown(markdown: Markdown) {
    listen_to(CONTENT_EVENT, move |output: EmittedMarkdown<String>| {
        markdown.set(output);
        false
    });
}

pub fn listen_to_config(conf: Config) {
    if conf.live_config_reload.get_untracked() {
        listen_to(CONFIG_EVENT, move |output: EmittedConfig| {
            let lch = output.live_config_reload;
            conf.update(output);
            !lch
        });
    }
}

pub fn key_bindings(conf: Config) {
    window_event_listener(ev::keydown, move |ke: ev::KeyboardEvent| {
        let code = ke.code();
        let keys = conf.keys.get_untracked();

        if code.eq(&keys.print) {
            window().print().unwrap_throw();
        }

        if code.eq(&keys.next_theme) {
            conf.next_theme();
        }

        if code.eq(&keys.prev_theme) {
            conf.prev_theme();
        }

        if code.eq(&keys.next_slide) {
            silent_invoke("next_slide");
        }

        if code.eq(&keys.prev_slide) {
            silent_invoke("prev_slide");
        }

        if code.eq(&keys.increase_fontsize) {
            conf.increase_font_size();
        }

        if code.eq(&keys.decrease_fontsize) {
            conf.decrease_font_size();
        }

        if code.eq(&keys.help) {
            let dialog: HtmlDialogElement = document()
                .get_element_by_id(HELP_ID)
                .unwrap()
                .dyn_into()
                .unwrap();
            dialog.show_modal().unwrap();
        }
    });
}
