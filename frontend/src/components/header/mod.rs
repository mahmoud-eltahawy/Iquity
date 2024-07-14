pub mod add_dropdown;
pub mod save_btn;

use yew::prelude::*;

use crate::components::tooltip::Tooltip;
use urlencoding::encode;
use yew_router::prelude::use_navigator;

use crate::{
    components::{
        header::{add_dropdown::AddFileDropdown, save_btn::SaveBtn},
        theme_card::ThemeDropdownItem,
    },
    contexts::markdown::use_markdown,
    icons::{PaletteIcon, RESPONSIVE_ICON_LG},
    Page,
};

pub const DOWNLOAD_ANCHOR_ID: AttrValue = AttrValue::Static("dl");
pub const PDF_DOWNLOAD_ANCHOR_ID: AttrValue = AttrValue::Static("pdf_dl");

#[function_component(Header)]
pub fn header() -> Html {
    let nav = use_navigator().unwrap();
    let home_cb = Callback::from(move |_| nav.replace(&Page::Home));

    let markdown = use_markdown().state();
    let encoded_md = encode(&markdown.text).to_string();

    let text_dl = format!("data:attachment/text,{}", encoded_md);

    let download_name = use_markdown().state().key;

    let dropdown_classes = classes!("dropdown", "dropdown-end");

    html! {
        <div class="navbar bg-base-300">
            <div class="navbar-start">
                <AddFileDropdown />
                <SaveBtn />
            </div>

            <div class="navbar-center">
                <btn class="btn btn-ghost normal-case font-display font-normal text-3xl" onclick={home_cb}>
                    {"Ubiquity"}
                </btn>
            </div>

            <div class="navbar-end">
                <div class={dropdown_classes.clone()}>
                    <Tooltip tip={"Themes"}>
                        <label tabindex="0" class="btn btn-ghost rounded-btn">
                            <PaletteIcon classes={RESPONSIVE_ICON_LG} />
                        </label>
                    </Tooltip>
                    <ul tabindex="0" class="menu dropdown-content p-2 shadow bg-base-200 rounded-box w-52">
                        <ThemeDropdownItem name={"aqua"} />
                        <ThemeDropdownItem name={"night"} />
                        <ThemeDropdownItem name={"synthwave"} />
                        <ThemeDropdownItem name={"winter"} />
                    </ul>
                </div>
            </div>
            <a id={DOWNLOAD_ANCHOR_ID} class="hidden" href={text_dl} download={download_name} target="_blank" />
        </div>
    }
}
