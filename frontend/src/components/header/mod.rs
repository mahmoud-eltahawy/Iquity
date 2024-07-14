pub mod add_dropdown;
pub mod desktop;
pub mod save_btn;

use yew::prelude::*;

use crate::components::header::desktop::DesktopHeader;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <DesktopHeader />
    }
}
