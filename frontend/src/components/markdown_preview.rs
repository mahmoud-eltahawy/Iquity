use leptos::{
    html::{article, div},
    prelude::*,
};

use crate::Markdown;

pub fn markdown_preview() -> impl IntoView {
    let markdown = use_context::<Markdown>().unwrap();

    let md = move || markdown.content.get();

    div()
        .attr("class", "overflow-auto")
        .child(article()
            .id("preview")
            .class("absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 p-5 prose-img:rounded-xl prose-pre:bg-base-300 prose-pre:text-base-content prose-pre:overflow-auto prose-code:bg-base-300 prose-code:px-[5.5px] prose-code:font-normal prose-code:rounded-[0.3125rem] prose-code:overflow-auto prose-a:no-underline prose-a:text-info print:block")
            .inner_html(md))
}
