use leptos::html::{dialog, div, p};
use leptos::prelude::*;

pub const HELP_ID: &str = "HELP";

pub fn help() -> impl IntoView {
    dialog()
        .id(HELP_ID)
        .class("modal").child(
            div()
            .class("modal-box grid grid-cols-1 gap-2 w-9/12 h-5/6 text-center border-2 rounded-lg p-5 prose-lg")
            .child((
                p().child("p => print to pdf"),
                p().child("j => next theme"),
                p().child("k => previous theme"),
                p().child("L => next slide"),
                p().child("H => previous slide"),
                p().child("= or + => increase font size"),
                p().child("- or _ => decrease font size"),
                p().child("? or / => show this help message"),
                p().child("esc => to hide this message"),
            )))
}
