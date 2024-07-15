pub mod header;
pub mod textarea;

use yew::prelude::*;

use crate::components::editor::textarea::EditorTextarea;

pub mod leptos_version {
    use crate::components::editor::textarea::leptos_version::EditorTextarea;
    use leptos::prelude::*;

    #[component]
    pub fn Editor() -> impl IntoView {
        view! {
            <div class="flex flex-col h-full">
                <EditorTextarea />
            </div>
        }
    }
}

#[function_component(Editor)]
pub fn editor() -> Html {
    html! {
        <div class="flex flex-col h-full">
            <EditorTextarea />
        </div>
    }
}
