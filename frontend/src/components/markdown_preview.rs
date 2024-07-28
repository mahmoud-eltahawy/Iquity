use config::EmittedMarkdown;
use leptos::{
    html::{article, div},
    prelude::*,
};

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

#[derive(Clone, Copy, Debug)]
pub struct Markdown {
    pub content: RwSignal<String>,
    pub current: RwSignal<usize>,
    pub len: RwSignal<usize>,
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
        if self.current.get_untracked() != current {
            self.current.set(current);
        }
        if self.len.get_untracked() != len {
            self.len.set(len);
        }
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
