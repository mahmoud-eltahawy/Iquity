use config::CONFIG_EVENT;
use config::{Action, EmittedConfig, EmittedMarkdown, InitConfig, KeyName, CONTENT_EVENT};
use futures::StreamExt;
use tauri_sys::{core::invoke, event::listen};

use leptos::{ev, prelude::*, spawn::spawn_local};

use serde::{de::DeserializeOwned, Serialize};
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
#[derive(Serialize)]
struct Empty {}

pub fn silent_invoke(action: &'static str) {
    spawn_local(async move {
        invoke::<()>(action, Empty {}).await;
    });
}

pub fn config_init(conf: Config) {
    spawn_local(async move {
        conf.set(invoke::<InitConfig>("conf_init", Empty {}).await);
    });
}

pub fn notify(title: &'static str, message: String) {
    #[derive(Serialize)]
    struct Content {
        title: &'static str,
        message: String,
    }
    spawn_local(async move {
        invoke::<()>("notify", Content { title, message }).await;
    });
}

pub fn export_html(html: String) {
    #[derive(Serialize)]
    struct Content {
        html: String,
    }
    spawn_local(async move {
        invoke::<()>("export_html", Content { html }).await;
    });
}

pub fn listen_to_markdown(markdown: Markdown) {
    listen_to(CONTENT_EVENT, move |output: EmittedMarkdown<String>| {
        markdown.set(output);
        false
    });
}

pub fn listen_to_config(conf: Config) {
    if *conf.live_config_reload.borrow() {
        listen_to(CONFIG_EVENT, move |output: EmittedConfig| {
            let lch = output.live_config_reload;
            conf.update(output);
            !lch
        });
    }
}

pub fn key_bindings(conf: Config) {
    window_event_listener(ev::keydown, move |ke: ev::KeyboardEvent| {
        let keys = conf.keys.borrow();
        let Some(action) = keys.get(&KeyName::from(ke.code().as_str())) else {
            return;
        };

        match action {
            Action::Print => window().print().unwrap_throw(),
            Action::NextTheme => conf.next_theme(),
            Action::PrevTheme => conf.prev_theme(),
            Action::NextSlide => silent_invoke("next_slide"),
            Action::PrevSlide => silent_invoke("prev_slide"),
            Action::IncreaseFontsize => conf.increase_font_size(),
            Action::DecreaseFontsize => conf.decrease_font_size(),
            Action::Help => {
                let dialog: HtmlDialogElement = document()
                    .get_element_by_id(HELP_ID)
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                dialog.show_modal().unwrap();
            }
            Action::SpitHtmlOut => {
                let html = document().first_element_child().unwrap().inner_html();

                let doc = document().create_element("div").unwrap();
                doc.set_inner_html(&html);
                let scripts = doc.get_elements_by_tag_name("script");
                for i in 0..scripts.length() {
                    let script = scripts.item(i).unwrap();
                    script.set_inner_html("");
                    let _ = script.remove_attribute("type");
                    let _ = script.remove_attribute("nonce");
                    //FIX : remove script itself instead of removing it's content and attributes;
                }
                let links = doc.get_elements_by_tag_name("link");
                for i in 0..links.length() {
                    let link = links.item(i).unwrap();
                    let _ = link.remove_attribute("rel");
                    let _ = link.remove_attribute("href");
                    let _ = link.remove_attribute("crossorigin");
                    let _ = link.remove_attribute("integrity");
                    let _ = link.remove_attribute("type");
                    let _ = link.remove_attribute("as");
                    //FIX : remove links instead of clearing it's attributes;
                }
                doc.get_elements_by_tag_name("progress")
                    .item(0)
                    .unwrap()
                    .remove();
                doc.get_elements_by_tag_name("dialog")
                    .item(0)
                    .unwrap()
                    .remove();
                let html = format!("<!DOCTYPE html><html>{}</html>", doc.inner_html());
                export_html(html);
            }
        }
    });
}
