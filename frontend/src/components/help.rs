use leptos::html::{dialog, div};
use leptos::prelude::*;

pub const HELP_ID: &str = "HELP_ID77";

pub fn help(html: RwSignal<String>) -> impl IntoView {
    let get_html = move || html.get();
    dialog()
        .id(HELP_ID)
        .class("modal").child(
            div()
            .class("modal-box grid grid-cols-1 gap-2 w-9/12 h-5/6 text-center border-2 rounded-lg p-5 prose-lg")
            .inner_html(get_html))
}
