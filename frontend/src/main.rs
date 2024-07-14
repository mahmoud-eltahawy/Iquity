pub mod components;
pub mod contexts;
pub mod icons;
pub mod pages;
pub mod tauri;

use contexts::config::ConfigProvider;

use leptos::context::provide_context;
use pages::home::Home;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::Page;

use crate::contexts::config::config_provider;
use crate::contexts::markdown::leptos_version::markdown_provider;
use crate::contexts::markdown::MarkdownProvider;
use crate::contexts::toasts::leptos_version::toaster_provider;
use crate::contexts::toasts::ToasterProvider;

#[function_component(App)]
fn app() -> Html {
    provide_context(config_provider());
    provide_context(markdown_provider());
    provide_context(toaster_provider());

    html! {
        <ConfigProvider>//DONE
            <ToasterProvider>//DONE
                <MarkdownProvider>//DONE
                    <BrowserRouter>
                        <Switch<Page> render={move |page| {
                            match page {
                                Page::Home => html!(<Home />),
                            }
                        }} />
                    </BrowserRouter>
                </MarkdownProvider>
            </ToasterProvider>
        </ConfigProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
