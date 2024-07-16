pub mod components;
pub mod contexts;
pub mod tauri;

use components::dual_view::{editor_view, preview_view};
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use crate::contexts::config::config_provider;
use crate::contexts::markdown::markdown_provider;

fn app() -> impl IntoView {
    let conf = config_provider();
    provide_context(conf);
    provide_context(markdown_provider());

    let theme = move || conf.get().theme;
    view! {
        <main
            data-theme=theme
            class="flex flex-col justify-between max-w-[calc(100svw)] print:hidden min-h-screen"
        >
        <Router>
            <Routes fallback=|| view!{"".to_string()}>
                <Route path=StaticSegment("editor") view=editor_view/>
                <Route path=StaticSegment("preview") view=preview_view/>
            </Routes>
        </Router>
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(app);
}
