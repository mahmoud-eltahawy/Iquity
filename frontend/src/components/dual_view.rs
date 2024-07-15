use crate::components::container::HalfWidthContainer;
use crate::components::editor::Editor;
use crate::components::markdown_preview::MarkdownPreview;
use leptos::prelude::*;

#[component]
pub fn DualView() -> impl IntoView {
    let dual_view_classes = "w-[calc(100vw-2.5rem)] flex flex-1 flex-row justify-center space-x-8 items-center h-[calc(100vh-8.5rem)]";

    view! {
        <div class=dual_view_classes>
            <HalfWidthContainer>
                <Editor />
            </HalfWidthContainer>
            <HalfWidthContainer>
                <MarkdownPreview />
            </HalfWidthContainer>
        </div>
    }
}
