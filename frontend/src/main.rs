pub mod components;
pub mod contexts;
pub mod icons;
pub mod pages;
pub mod tauri;

use leptos::prelude::*;

use crate::components::dual_view::DualView;
use crate::contexts::config::config_provider;
use crate::contexts::markdown::leptos_version::markdown_provider;
use crate::contexts::toasts::leptos_version::toaster_provider;
use crate::pages::background::Background;

#[component]
fn App() -> impl IntoView {
    provide_context(config_provider());
    provide_context(markdown_provider());
    provide_context(toaster_provider());

    view! {
        <Background>
            <div class="h-[calc(100vh-4rem)] flex flex-col content-center align-center items-center justify-center">
                <DualView />
            </div>
        </Background>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(App);
}
