use leptos::html::{dialog, div};
use leptos::prelude::*;

pub const HELP_ID: &str = "HELP_ID77";

pub fn help(html: RwSignal<String>) -> impl IntoView {
    let get_html = move || html.get();
    dialog().id(HELP_ID).child(div().inner_html(get_html))
}
