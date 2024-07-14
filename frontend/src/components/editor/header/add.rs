use crate::components::modals::add_image::ADD_IMAGE_MODAL_ID;
use crate::components::modals::add_link::ADD_LINK_MODAL_ID;
use crate::components::modals::table::TABLE_MODAL_ID;
use crate::icons::{ImageIcon, LinkIcon, TableIcon};
use yew::prelude::*;

mod leptos_version {
    use crate::{
        components::modals::{
            add_image::leptos_version::ADD_IMAGE_MODAL_ID,
            add_link::leptos_version::ADD_LINK_MODAL_ID, table::leptos_version::TABLE_MODAL_ID,
        },
        icons::leptos_version::{ImageIcon, LinkIcon, TableIcon},
    };
    use leptos::prelude::*;

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
