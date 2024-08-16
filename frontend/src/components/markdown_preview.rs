use std::{cell::RefCell, rc::Rc};

use config::EmittedMarkdown;
use leptos::{
    html::{article, div},
    prelude::*,
};
use tachys::dom::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;

pub fn markdown_preview(port: Rc<RefCell<u16>>) -> impl IntoView {
    let markdown = use_context::<Markdown>().unwrap();

    let md = move || markdown.content.get();

    Effect::new(move |_| {
        let _ = md();
        let images = document().get_elements_by_tag_name("img");
        for i in 0..images.length() {
            let image: HtmlImageElement = images.item(i).unwrap().dyn_into().unwrap();
            image.get_attribute("src").inspect(|x| {
                if !x.starts_with("http") {
                    let content = format!("http://localhost:{}/{}", port.borrow(), x);
                    image.set_src(&content);
                }
            });
        }
    });

    div().child(article().id("preview").inner_html(md))
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
