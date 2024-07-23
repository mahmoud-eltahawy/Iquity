use std::collections::BTreeMap;

use config::EmittedMarkdown;
use leptos::{
    html::{article, div},
    prelude::*,
};

#[derive(Clone, Copy, Debug)]
pub struct Markdown {
    cache: RwSignal<BTreeMap<usize, Box<String>>>,
    content: RwSignal<Box<String>>,
    pub current: RwSignal<usize>,
    pub len: RwSignal<usize>,
}

impl Markdown {
    pub fn clear_cache(&self) {
        self.cache.update_untracked(|xs| xs.clear());
    }
    pub fn cache_call(&self, current: usize) -> bool {
        match self.cache.with_untracked(|xs| xs.get(&current).cloned()) {
            Some(content) => {
                self.content.set(content);
                if self.current.get_untracked() != current {
                    self.current.set(current);
                }
                true
            }
            None => false,
        }
    }
    pub fn cache_set(&self, index: usize, content: Box<String>) {
        self.cache.update_untracked(|xs| xs.insert(index, content));
    }
    pub fn set(&self, EmittedMarkdown { current, len }: EmittedMarkdown, content: Box<String>) {
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
            cache: RwSignal::new(BTreeMap::new()),
            content: RwSignal::new(Box::new(String::default())),
            current: RwSignal::new(0),
            len: RwSignal::new(0),
        }
    }
}

pub fn markdown_preview() -> impl IntoView {
    let markdown = use_context::<Markdown>().unwrap();

    let md = move || markdown.content.get().to_string();

    div()
        .class(
            "flex flex-col h-full overflow-visible scroll-smooth h-screen w-screen p-5",
        )
        .child(
            div()
                .attr("class", "overflow-auto")
                .child(article()
                    .id("preview")
                    .class("prose prose-img:rounded-xl prose-pre:bg-base-300 prose-pre:text-base-content prose-pre:overflow-auto prose-code:bg-base-300 prose-code:px-[5.5px] prose-code:font-normal prose-code:rounded-[0.3125rem] prose-code:overflow-auto prose-a:no-underline prose-a:text-info print:block")
                    .inner_html(md)),
        )
}
