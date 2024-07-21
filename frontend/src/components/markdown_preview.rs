use leptos::{
    html::{article, div},
    prelude::*,
};
use markdown::{self, CompileOptions, Options, ParseOptions};

use crate::Markdown;

pub fn markdown_preview() -> impl IntoView {
    let markdown = use_context::<Markdown>().unwrap();

    let md = move || {
        let compile = CompileOptions {
            allow_dangerous_html: true,
            allow_dangerous_protocol: true,
            ..CompileOptions::default()
        };
        let parse = ParseOptions::gfm();
        let options = Options { compile, parse };
        let md = markdown.content.get();
        markdown::to_html_with_options(&md, &options).unwrap()
    };

    div()
        .class(
            "flex flex-col h-full overflow-visible scroll-smooth h-screen w-screen p-5",
        )
        .child(
            div()
                .attr("class", "overflow-auto")
                .child(article().id("preview").class("prose prose-img:rounded-xl prose-pre:bg-base-300 prose-pre:text-base-content prose-pre:overflow-auto prose-code:bg-base-300 prose-code:px-[5.5px] prose-code:font-normal prose-code:rounded-[0.3125rem] prose-code:overflow-auto prose-a:no-underline prose-a:text-info print:block").inner_html(md)),
        )
}
