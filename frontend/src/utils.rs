use config::{EmittedConfig, EmittedMarkdown, CONTENT_EVENT};
use config::{GlobalConfig, CONFIG_EVENT};
use futures::StreamExt;
use gloo::utils::{document, window};
use markdown::{CompileOptions, Options, ParseOptions};
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

pub fn listen_to_markdown(md: Markdown) {
    listen_to(CONTENT_EVENT, move |output: EmittedMarkdown| {
        #[derive(Serialize)]
        struct Index {
            index: usize,
        }
        if !md.cache_call(output.current) {
            spawn_local(async move {
                let content = invoke::<String>(
                    "get_md",
                    Index {
                        index: output.current,
                    },
                )
                .await;

                let options = compile_options();
                let content = Box::new(markdown::to_html_with_options(&content, &options).unwrap());
                md.cache_set(output.current, content.clone());
                md.set(output, content);
            });
        };
        false
    });
}

fn compile_options() -> Options {
    let compile = CompileOptions {
        allow_dangerous_html: true,
        allow_dangerous_protocol: true,
        ..CompileOptions::default()
    };
    let parse = ParseOptions::gfm();
    let options = Options { compile, parse };
    options
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
