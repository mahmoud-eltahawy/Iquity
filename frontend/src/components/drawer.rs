use yew::prelude::*;

use crate::contexts::config::use_config;

#[derive(Debug, PartialEq, Properties)]
pub struct DrawerProps {
    pub children: Children,
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    let theme = use_config().state().theme;

    let drawer_classes = classes!(
        "flex",
        "flex-col",
        "h-full",
        "bg-base-300",
        "py-2",
        "3xl:w-[10%]",
        "2xl:w-[15%]",
        "xl:w-[20%]",
        "lg:w-[30%]",
        "md:w-[40%]",
        "sm:w-[50%]",
        "xs:w-[60%]",
        "w-[60%]",
    );

    html! {
        <div data-theme={theme} class="drawer print:hidden">
            <input id="drawer-input" type="checkbox" class="drawer-toggle" />
            <div class="drawer-content">
                { props.children.clone() }
            </div>
            <div class="drawer-side">
                <label for="drawer-input" class="drawer-overlay"></label>
                <div class={drawer_classes}>
                <h1 class="mt-2 mb-3 text-2xl font-display font-bold tracking-wide self-center">{"Ubiquity"}</h1>
            </div>
            </div>
        </div>
    }
}
