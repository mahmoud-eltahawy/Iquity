use crate::components::container::container;
use crate::components::editor::editor;
use crate::components::markdown_preview::markdown_preview;
use leptos::html::div;
use leptos::prelude::*;

use super::container::style_container;

pub fn editor_view() -> impl IntoView {
    style_container(
    div()
        .class("w-[calc(100vw-2.5rem)] flex flex-1 flex-row justify-center space-x-8 items-center h-[calc(100vh-8.5rem)]")
        .child(
            container(editor()),
        ))
}

pub fn preview_view() -> impl IntoView {
    style_container(
    div()
        .class("w-[calc(100vw-2.5rem)] flex flex-1 flex-row justify-center space-x-8 items-center h-[calc(100vh-8.5rem)]")
        .child(
            container(markdown_preview())
        ))
}
