use crate::components::container::Container;
use crate::components::editor::Editor;
use crate::components::markdown_preview::MarkdownPreview;
use crate::contexts::config::use_config;
use config::View;
use yew::prelude::*;

#[function_component(SingleView)]
pub fn single_view() -> Html {
    let preview = use_config().state().view == View::Preview;

    html! {
        <Container>
            if preview {
                <MarkdownPreview />
            } else {
                <Editor />
            }
        </Container>
    }
}
