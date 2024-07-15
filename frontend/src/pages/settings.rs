use crate::components::theme_card::ThemeCard;
use crate::contexts::config::THEMES;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct SettingsPageProps {
    pub children: Children,
}

#[function_component(SettingsPage)]
pub fn settings_page(props: &SettingsPageProps) -> Html {
    let page_classes = classes!(
        "flex",
        "justify-center",
        "bg-base-100",
        "h-[calc(100vh-4rem)]"
    );

    html! {
        <div class={page_classes}>
            { props.children.clone() }
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct SettingsContainerProps {
    pub children: Children,
}

#[function_component(SettingsContainer)]
pub fn settings_container(props: &SettingsContainerProps) -> Html {
    let container_classes = classes!(
        "flex",
        "flex-col",
        "w-[97.5vw]",
        "2xl:w-[50vw]",
        "xl:w-[50vw]",
        "lg:w-[85vw]",
        "md:w-[90vw]",
        "sm:w-[95vw]",
        "my-[3vh]",
        "px-6",
        "overscroll-contain",
        "overflow-visible",
        "overflow-y-auto",
        "gap-16"
    );

    html! {
        <div class={container_classes}>
            { props.children.clone() }
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct SettingsHeaderProps {
    pub text: AttrValue,
}

#[function_component(SettingsHeader)]
pub fn settings_header(props: &SettingsHeaderProps) -> Html {
    let header_classes = classes!("font-display", "text-3xl");

    html! {
        <h1 class={header_classes}>
            { &props.text }
        </h1>
    }
}

#[function_component(ThemeSettings)]
pub fn theme_settings() -> Html {
    let mut theme_btns_html: Vec<Html> = Vec::new();

    for theme in THEMES {
        let att = AttrValue::from(*theme);
        let html = html! { <ThemeCard name={att} /> };
        theme_btns_html.push(html);
    }

    html! {
        <div class="flex flex-col gap-3">
            <SettingsHeader text={"Theme"}/>
            <div class="flex flex-wrap flex-row gap-4">
                { for theme_btns_html }
            </div>
        </div>
    }
}
