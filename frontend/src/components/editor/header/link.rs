use crate::{
    components::{modals::add_link::ADD_LINK_MODAL_ID, tooltip::Tooltip},
    icons::LinkIcon,
};
use yew::prelude::*;

use super::HeaderBtnProps;

#[function_component(AddLinkBtn)]
pub fn add_link_btn(props: &HeaderBtnProps) -> Html {
    html! {
        <Tooltip tip={"Add link"}>
            <label for={&ADD_LINK_MODAL_ID} class={props.btn_classes}>
                <LinkIcon />
            </label>
        </Tooltip>
    }
}
