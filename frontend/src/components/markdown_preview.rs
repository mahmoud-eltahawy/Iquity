use leptos::{
    html::{article, div},
    prelude::*,
};
use markdown::{self, CompileOptions, Options, ParseOptions};

use crate::{
    local_config::{Config, THEMES, THEMES_SIZE},
    Markdown,
};

use super::notification;

pub fn markdown_preview() -> impl IntoView {
    let markdown = use_context::<Markdown>().unwrap();
    let conf = use_context::<Config>().unwrap();

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

    let class = move || {
        conf.font_size.get() + " prose prose-img:rounded-xl prose-pre:bg-base-300 prose-pre:text-base-content prose-pre:overflow-auto prose-code:bg-base-300 prose-code:px-[5.5px] prose-code:font-normal prose-code:rounded-[0.3125rem] prose-code:overflow-auto prose-a:no-underline prose-a:text-info print:block"
    };

    let theme = move || THEMES[conf.theme_index.get() % THEMES_SIZE];

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
        notification::notification(theme),
    )
}
