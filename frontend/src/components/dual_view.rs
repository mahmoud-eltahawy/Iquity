use crate::components::container::half_width_container;
use crate::components::editor::editor;
use crate::components::markdown_preview::markdown_preview;
use leptos::html::div;
use leptos::prelude::*;

pub fn dual_view() -> impl IntoView {
    div()
        .class("w-[calc(100vw-2.5rem)] flex flex-1 flex-row justify-center space-x-8 items-center h-[calc(100vh-8.5rem)]")
        .child(
            half_width_container(editor())
            // half_width_container(markdown_preview()))
        )
}
