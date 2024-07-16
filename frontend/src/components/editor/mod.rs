use crate::contexts::markdown::Markdown;
use config::Config;
use gloo::utils::document;
use leptos::ev::{Event, KeyboardEvent};
use leptos::html::{div, textarea};
use leptos::tachys::dom::event_target_value;
use leptos::{ev, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, HtmlTextAreaElement};
pub const EDITOR_ID: &str = "editor";

pub fn editor() -> impl IntoView {
    let markdown_ctx = use_context::<RwSignal<Markdown>>().unwrap();
    let conf = use_context::<RwSignal<Config>>().unwrap();

    let md_text = move || markdown_ctx.get().text;

    let oninput = move |event: Event| {
        let text = event_target_value(&event);
        markdown_ctx.update(|x| {
            x.update_text(text).unwrap_or_default();
        });
    };

    let key_check = move |key_event: KeyboardEvent| {
        if key_event.key().eq("Tab") {
            key_event.prevent_default();
            let text_area: HtmlTextAreaElement = document()
                .get_element_by_id(&EDITOR_ID)
                .unwrap()
                .dyn_into()
                .unwrap();
            let mut current_value = text_area.value();

            if let Some(end) = text_area.selection_end().unwrap() {
                let end_usize = end as usize;
                current_value.insert_str(end_usize, "    ");
                text_area.set_value(&current_value);
                text_area.set_selection_end(Some(end + 4)).unwrap();
            } else {
                current_value.push_str("    ");
                text_area.set_value(&current_value);
                text_area.set_selection_end(Some(4)).unwrap();
            }
            markdown_ctx.update(|x| {
                x.update_text(current_value).unwrap_or_default();
            });
        }

        if key_event.ctrl_key() && key_event.key().eq_ignore_ascii_case("Z") {
            key_event.prevent_default();
            let html_doc: HtmlDocument = document().dyn_into().unwrap();
            html_doc.exec_command("undo").unwrap();
        }

        if key_event.ctrl_key() && key_event.key().eq_ignore_ascii_case("Y") {
            key_event.prevent_default();
            let html_doc: HtmlDocument = document().dyn_into().unwrap();
            html_doc.exec_command("redo").unwrap();
        }
    };

    let class = move || {
        conf.get().md_input_font_size + "textarea bg-transparent whitespace-pre-wrap scroll-smooth font-mono resize-none border-none outline-none focus:outline-none w-full h-full overflow-y-auto"
    };

    div().class("flex flex-col h-full").child(
        textarea()
            .attr("id", EDITOR_ID)
            .attr("spellcheck", "false")
            .attr("class", class)
            .on(ev::keydown, key_check)
            .on(ev::input, oninput)
            .child(md_text),
    )
}
