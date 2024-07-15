use crate::contexts::config::use_config;
use crate::contexts::{
    markdown::{use_markdown, Markdown},
    toasts::{err_modal, use_toaster},
};
use gloo::console::debug;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, HtmlTextAreaElement};
use yew::prelude::*;

pub mod leptos_version {
    use crate::contexts::markdown::leptos_version::Markdown;
    use config::Config;
    use gloo::console::debug;
    use gloo::utils::document;
    use leptos::ev::{Event, KeyboardEvent};
    use leptos::html::textarea;
    use leptos::{ev, prelude::*};
    use wasm_bindgen::JsCast;
    use web_sys::{HtmlDocument, HtmlTextAreaElement};
    pub const EDITOR_ID: &str = "editor";

    #[component]
    pub fn EditorTextarea() -> impl IntoView {
        let markdown_ctx = use_context::<RwSignal<Markdown>>().unwrap();
        let conf = use_context::<RwSignal<Config>>().unwrap();

        let md_text = move || markdown_ctx.get().text;

        let oninput = move |event: Event| {
            // let event: Event = input_event.clone().dyn_into().unwrap();
            let editor: HtmlTextAreaElement = event.target().unwrap().dyn_into().unwrap();
            let text_area_str = editor.value();

            debug!(r#"Markdown Browser JS String: {}"#, &text_area_str);

            let text = text_area_str;
            let key = markdown_ctx.get().key;
            let md = Markdown::from(text, key);
            markdown_ctx.update(|x| {
                x.update(md).unwrap_or_default();
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
                let key = markdown_ctx.get().key;
                let md = Markdown::from(current_value, key);
                markdown_ctx.update(|x| {
                    x.update(md).unwrap_or_default();
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

        textarea()
            .attr("id", EDITOR_ID)
            .attr("spellcheck", "false")
            .attr("class", class)
            .attr("value", md_text)
            .on(ev::keydown, key_check)
            .on(ev::input, oninput)
    }
}

pub const EDITOR_ID: AttrValue = AttrValue::Static("editor");

#[function_component(EditorTextarea)]
pub fn editor_textarea() -> Html {
    let md_text = use_markdown().state().text;

    let markdown_ctx = use_markdown();
    let toaster = use_toaster();
    let oninput = Callback::from(move |event: InputEvent| {
        // let event: Event = input_event.clone().dyn_into().unwrap();
        let editor: HtmlTextAreaElement = event.target().unwrap().dyn_into().unwrap();
        let text_area_str = editor.value();

        debug!(r#"Markdown Browser JS String: {}"#, &text_area_str);

        let text = AttrValue::from(text_area_str);
        let key = markdown_ctx.state().key;
        let md = Markdown::from(text, key);
        markdown_ctx
            .update_markdown(md)
            .unwrap_or_else(|err| err_modal(err, toaster.clone()));
    });

    let markdown_ctx = use_markdown();
    let toaster = use_toaster();
    let key_check = Callback::from(move |key_event: KeyboardEvent| {
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
            let key = markdown_ctx.state().key;
            let md = Markdown::from(AttrValue::from(current_value), key);
            markdown_ctx
                .update_markdown(md)
                .unwrap_or_else(|err| err_modal(err, toaster.clone()));
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
    });

    let node_ref: NodeRef = NodeRef::default();
    let node_ref_clone = node_ref.clone();

    // Some browsers do not accept strings inside of a textarea element.
    use_effect(move || {
        let text_area = node_ref_clone.cast::<HtmlTextAreaElement>().unwrap();
        text_area.set_value(md_text.as_str());
    });

    let font_size = use_config().state().md_input_font_size;

    let classes = classes!(
        "textarea",
        "bg-transparent",
        "whitespace-pre-wrap",
        "scroll-smooth",
        font_size,
        "font-mono",
        "resize-none",
        "border-none",
        "outline-none",
        "focus:outline-none",
        "w-full",
        "h-full",
        "overflow-y-auto",
    );

    html! {
        <textarea ref={node_ref} id={EDITOR_ID} onkeydown={key_check} oninput={oninput} spellcheck={"false"}
            class={classes}>
        // Do NOT put strings here some browsers won't process them.
        </textarea>
    }
}
