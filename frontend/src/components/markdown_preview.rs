use std::time::Duration;

use config::Config;
use gloo::utils::window;
use gloo_timers::future::sleep;
use leptos::{
    either::Either,
    ev::{self, KeyboardEvent},
    html::{article, button, div, i, p, span},
    prelude::*,
    spawn::spawn_local,
};
use markdown::{self, CompileOptions, Options, ParseOptions};
use wasm_bindgen::UnwrapThrowExt;

use crate::Markdown;

fn notification(theme: impl Fn() -> &'static str + 'static) -> impl IntoView {
    let messages = RwSignal::new(Vec::new());
    let close = move || {
        messages.update(|xs| {
            xs.pop();
        });
    };

    Effect::new(move |_| {
        messages.update(|xs| xs.insert(0, theme()));
        spawn_local(async move {
            sleep(Duration::from_secs(1)).await;
            close();
        });
    });

    move || {
        if !messages.get().is_empty() {
            Either::Left(  button()
        .on(ev::click, move |_| close())
        .class("fixed right-4 top-4 z-50 rounded-md bg-green-500 px-4 py-2 text-white transition hover:bg-green-600")
        .child(div()
            .class("flex items-center space-x-2")
            .child((span()
                .class("text-3xl")
                .child(i()
                    .class("bx bx-check")),
                p()
                    .class("font-bold")
                    .child(move || messages.get().first().unwrap().to_string())))))
        } else {
            Either::Right(())
        }
    }
}

pub fn markdown_preview() -> impl IntoView {
    let markdown = use_context::<RwSignal<Markdown>>().unwrap();
    let conf = use_context::<RwSignal<Config>>().unwrap();

    let md = move || {
        let compile = CompileOptions {
            allow_dangerous_html: true,
            allow_dangerous_protocol: true,
            ..CompileOptions::default()
        };
        let parse = ParseOptions::gfm();
        let options = Options { compile, parse };
        let m = markdown.get();
        markdown::to_html_with_options(&m.0, &options).unwrap()
    };

    let class = move || {
        conf.get().font_size + " prose prose-img:rounded-xl prose-pre:bg-base-300 prose-pre:text-base-content prose-pre:overflow-auto prose-code:bg-base-300 prose-code:px-[5.5px] prose-code:font-normal prose-code:rounded-[0.3125rem] prose-code:overflow-auto prose-a:no-underline prose-a:text-info print:block"
    };

    let theme = move || conf.get().theme();

    window_event_listener(ev::keydown, move |ke: KeyboardEvent| {
        if ke.code().eq("F1") {
            window().print().unwrap_throw();
        }

        if ke.code().eq("F2") {
            conf.update(|x| x.next_theme());
        }

        if ke.code().eq("F3") {
            conf.update(|x| x.prev_theme());
        }

        if ke.code().eq("F4") {
            conf.update(|x| x.decrease_font_size());
        }

        if ke.code().eq("F5") {
            conf.update(|x| x.increase_font_size());
        }
    });

    (
        div()
            .attr(
                "class",
                "flex flex-col h-full overflow-visible scroll-smooth h-screen w-screen p-5",
            )
            .attr("data-theme", theme)
            .child(
                div()
                    .attr("class", "overflow-auto")
                    .child(article().id("preview").class(class).inner_html(md)),
            ),
        notification(theme),
    )
}
