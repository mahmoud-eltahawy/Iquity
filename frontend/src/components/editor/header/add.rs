use crate::components::modals::add_image::ADD_IMAGE_MODAL_ID;
use crate::components::modals::add_link::ADD_LINK_MODAL_ID;
use crate::components::modals::table::TABLE_MODAL_ID;
use crate::contexts::config::use_config;
use crate::icons::{ImageIcon, LinkIcon, PlusIcon, TableIcon};
use yew::prelude::*;

use super::HeaderBtnProps;

mod leptos_version {
    use crate::{
        components::modals::{
            add_image::leptos_version::ADD_IMAGE_MODAL_ID,
            add_link::leptos_version::ADD_LINK_MODAL_ID, table::leptos_version::TABLE_MODAL_ID,
        },
        icons::leptos_version::{ImageIcon, LinkIcon, PlusIcon, TableIcon},
    };
    use config::Config;
    use leptos::prelude::*;

    #[component]
    pub fn AddDropdown(btn_classes: String) -> impl IntoView {
        let conf = use_context::<RwSignal<Config>>().unwrap();
        let plus_class = move || {
            if conf.get().mobile_ui {
                "dropdown-end"
            } else {
                ""
            }
        };

        let class = move || "dropdown".to_string() + plus_class();

        view! {
            <div class=class>
                <label class=btn_classes id="add_file_dropdown" tabindex="0" class="btn btn-ghost">
                    <PlusIcon />
                </label>
                <div class="dropdown-content z-[1] menu p-2 shadow bg-base-200 rounded-box w-52 lg:w-max">
                    <ul tabindex="0">
                        <LinkBtn />
                        <ImageBtn />
                        <Table />
                    </ul>
                </div>
            </div>
        }
    }

    #[component]
    pub fn LinkBtn() -> impl IntoView {
        view! {
            <li>
                <label for=ADD_LINK_MODAL_ID>
                    <LinkIcon />
                    {"Link"}
                </label>
            </li>
        }
    }

    #[component]
    pub fn ImageBtn() -> impl IntoView {
        view! {
            <li>
                <label for=ADD_IMAGE_MODAL_ID>
                    <ImageIcon />
                    {"Image"}
                </label>
            </li>
        }
    }

    #[component]
    pub fn Table() -> impl IntoView {
        view! {
            <li>
                <label for=TABLE_MODAL_ID>
                    <TableIcon />
                    {"Table"}
                </label>
            </li>
        }
    }
}

#[function_component(AddDropdown)]
pub fn add_dropdown_btn(props: &HeaderBtnProps) -> Html {
    let mut dropdown_classes = classes!("dropdown");
    if use_config().is_mobile_ui() {
        dropdown_classes.push("dropdown-end");
    }

    html! {
        <div class={dropdown_classes}>
            <label class={props.btn_classes} id="add_file_dropdown" tabindex="0" class="btn btn-ghost">
                <PlusIcon />
            </label>
            <div class="dropdown-content z-[1] menu p-2 shadow bg-base-200 rounded-box w-52 lg:w-max">
                <ul tabindex="0">
                    <LinkBtn />
                    <ImageBtn />
                    <Table />
                </ul>
            </div>
        </div>
    }
}

#[function_component(LinkBtn)]
pub fn link_btn() -> Html {
    html! {
        <li>
            <label for={&ADD_LINK_MODAL_ID}>
                <LinkIcon />
                {"Link"}
            </label>
        </li>
    }
}

#[function_component(ImageBtn)]
pub fn image_btn() -> Html {
    html! {
        <li>
            <label for={&ADD_IMAGE_MODAL_ID}>
                <ImageIcon />
                {"Image"}
            </label>
        </li>
    }
}

#[function_component(Table)]
pub fn table() -> Html {
    html! {
        <li>
            <label for={&TABLE_MODAL_ID}>
                <TableIcon />
                {"Table"}
            </label>
        </li>
    }
}
