use crate::components::tooltip::Tooltip;
use crate::contexts::config::use_config;
use crate::contexts::toasts::{err_modal, use_toaster};
use crate::icons::FontIncreaseIcon;
use yew::prelude::*;

use super::HeaderBtnProps;

#[function_component(FontIncreaseBtn)]
pub fn font_increase_btn(props: &HeaderBtnProps) -> Html {
    let config_ctx = use_config();
    let toaster = use_toaster();
    let increase_font_size = Callback::from(move |_| {
        let toaster = toaster.clone();
        config_ctx
            .increase_font_size()
            .unwrap_or_else(|err| err_modal(err, toaster));
    });

    html! {
        <Tooltip tip={"Increase font size"}>
            <btn onclick={increase_font_size} class={props.btn_classes}>
                <FontIncreaseIcon />
            </btn>
        </Tooltip>
    }
}
