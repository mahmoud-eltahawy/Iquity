pub mod header;
pub mod textarea;

use yew::prelude::*;

use crate::components::editor::{header::EditorHeader, textarea::EditorTextarea};

#[function_component(Editor)]
pub fn editor() -> Html {
    html! {
        <div class="flex flex-col h-full">
            <EditorHeader />
            <EditorTextarea />
        </div>
    }
}
