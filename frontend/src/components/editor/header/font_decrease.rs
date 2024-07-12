use crate::components::tooltip::Tooltip;
use crate::contexts::config::use_config;
use crate::contexts::toasts::{err_modal, use_toaster};
use crate::icons::FontDecreaseIcon;
use yew::prelude::*;

use super::HeaderBtnProps;

#[function_component(FontDecreaseBtn)]
pub fn font_decrease_btn(props: &HeaderBtnProps) -> Html {
    let config_ctx = use_config();
    let toaster = use_toaster();
    let decrease_font_size = Callback::from(move |_| {
        let toaster = toaster.clone();
        config_ctx
            .decrease_font_size()
            .unwrap_or_else(|err| err_modal(err, toaster));
    });

    html! {
        <Tooltip tip={"Decrease font size"}>
            <btn onclick={decrease_font_size} class={props.btn_classes}>
                <FontDecreaseIcon />
            </btn>
        </Tooltip>
    }
}
