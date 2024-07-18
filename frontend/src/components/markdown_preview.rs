use config::{Config, THEMES};
use gloo::utils::window;
use leptos::{
    ev::{self, KeyboardEvent},
    html::{article, div},
    prelude::*,
};
use markdown::{self, CompileOptions, Options, ParseOptions};
use wasm_bindgen::UnwrapThrowExt;

use crate::Markdown;

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
        conf.get().md_preview_font_size + "prose prose-img:rounded-xl prose-pre:bg-base-300 prose-pre:text-base-content prose-pre:overflow-auto prose-code:bg-base-300 prose-code:px-[5.5px] prose-code:font-normal prose-code:rounded-[0.3125rem] prose-code:overflow-auto prose-a:no-underline prose-a:text-info print:block"
    };

    let theme = move || conf.get().theme;

    window_event_listener(ev::keydown, move |ke: KeyboardEvent| {
        if ke.code().eq("F1") {
            window().print().unwrap_throw();
        }

        if ke.code().eq("F2") {
            let mut cycle = THEMES.iter().cycle();
            conf.update(|x| loop {
                let next_theme = cycle.next().unwrap().to_string();
                if cycle.next() == Some(&x.theme.as_str()) {
                    x.theme = next_theme;
                    break;
                }
            });
            //
        }
    });

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
        )
}
