pub mod components;
pub mod contexts;
pub mod icons;
pub mod pages;
pub mod tauri;

use pages::home::Home;

use leptos::prelude::*;

use crate::contexts::config::config_provider;
use crate::contexts::markdown::leptos_version::markdown_provider;
use crate::contexts::toasts::leptos_version::toaster_provider;

#[component]
fn App() -> impl IntoView {
    provide_context(config_provider());
    provide_context(markdown_provider());
    provide_context(toaster_provider());

    view! {
        <Home />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(App);
}
