mod components;
mod local_config;
mod utils;

use std::ops::Deref;

use components::help::help;
use gloo::utils::window;
use leptos::{either::Either, ev, prelude::*};
use local_config::Config;
use utils::listen_to;
use wasm_bindgen::UnwrapThrowExt;

use crate::components::markdown_preview::markdown_preview;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Markdown(pub RwSignal<String>);

impl Default for Markdown {
    fn default() -> Self {
        Markdown(RwSignal::new(String::default()))
    }
}

impl Deref for Markdown {
    type Target = RwSignal<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn app() -> impl IntoView {
    let markdown = Markdown::default();
    let conf = Config::default();
    let help_message = RwSignal::new(false);

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

        if code.eq("Minus") {
            conf.decrease_font_size();
        }

        if code.eq("Equal") {
            conf.increase_font_size();
        }

        if help_message.get_untracked() {
            if code == "Escape" {
                help_message.set(false);
            }
        }

        if code.eq("Slash") {
            help_message.set(true);
        }
    });

    provide_context(conf);
    provide_context(markdown);

    listen_to("content", move |payload| {
        markdown.update(|content| {
            *content = payload;
        });
    });
    (markdown_preview(), move || {
        if help_message.get() {
            Either::Left(help())
        } else {
            Either::Right(())
        }
    })
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(app);
}
