use crate::components::tooltip::Tooltip;
use crate::contexts::{
    markdown::{use_markdown, Markdown},
    toasts::{err_modal, use_toaster},
};
use crate::icons::QuoteIcon;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use super::HeaderBtnProps;

#[function_component(QuoteBtn)]
pub fn quote_btn(props: &HeaderBtnProps) -> Html {
    let md_state = use_markdown();
    let toaster = use_toaster();
    let quote = Callback::from(move |_mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document()
            .get_element_by_id("editor")
            .unwrap()
            .dyn_into()
            .unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap()
            && let Some(end) = text_area.selection_end().unwrap()
        {
            let start_usize = start as usize;
            let _end_usize = end as usize;

            current_value.insert_str(start_usize, "> ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 2)).unwrap();
        } else {
            current_value.push_str("> ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(2)).unwrap();
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state
            .update_markdown(md)
            .unwrap_or_else(|err| err_modal(err, toaster.clone()));
    });

    html! {
        <Tooltip tip={"Quote"}>
            <btn onclick={quote} class={props.btn_classes}>
                <QuoteIcon />
            </btn>
        </Tooltip>
    }
}
