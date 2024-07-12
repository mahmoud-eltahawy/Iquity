pub mod add_dropdown;
pub mod desktop;
pub mod mobile;
pub mod save_btn;

use yew::prelude::*;

use crate::{
    components::header::{desktop::DesktopHeader, mobile::MobileHeader},
    contexts::config::use_config,
};

#[function_component(Header)]
pub fn header() -> Html {
    let mobile_ui = use_config().is_mobile_ui();

    html! {
        if mobile_ui {
            <MobileHeader />
        } else {
            <DesktopHeader />
        }
    }
}
