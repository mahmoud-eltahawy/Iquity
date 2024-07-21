mod components;
mod local_config;
mod utils;

use components::help::help;
use config::EmittedMarkdown;
use gloo::utils::window;
use leptos::{either::Either, ev, prelude::*};
use local_config::Config;
use utils::{listen_to_content, silent_invoke};
use wasm_bindgen::UnwrapThrowExt;

use crate::components::markdown_preview::markdown_preview;

#[derive(Clone, Copy, Debug)]
pub struct Markdown {
    content: RwSignal<String>,
    current: RwSignal<usize>,
    len: RwSignal<usize>,
}

impl From<EmittedMarkdown<String>> for Markdown {
    fn from(
        EmittedMarkdown {
            current,
            len,
            content,
        }: EmittedMarkdown<String>,
    ) -> Self {
        Self {
            content: RwSignal::new(content),
            current: RwSignal::new(current),
            len: RwSignal::new(len),
        }
    }
}

impl Markdown {
    pub fn set(
        &self,
        EmittedMarkdown {
            current,
            len,
            content,
        }: EmittedMarkdown<String>,
    ) {
        self.content.set(content);
        self.current.set(current);
        self.len.set(len);
    }
}

impl Default for Markdown {
    fn default() -> Self {
        Markdown {
            content: RwSignal::new(String::default()),
            current: RwSignal::new(0),
            len: RwSignal::new(0),
        }
    }
}

pub fn app() -> impl IntoView {
    let markdown = Markdown::default();
    listen_to_content(markdown);
    silent_invoke("md_init");

    let conf = Config::default();
    let help_message = RwSignal::new(false);

    provide_context(conf);
    provide_context(markdown);

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

        if help_message.get_untracked() && code == "Escape" {
            help_message.set(false);
        }

        if code.eq("Slash") {
            help_message.set(true);
        }
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
