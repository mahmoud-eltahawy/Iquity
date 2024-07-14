use error::UbiquityError;
use yew::prelude::*;

use crate::{contexts::toasts::use_toaster, icons::CloseIcon};

pub static TOASTER_ID: &str = "toaster";

pub mod leptos_version {
    use crate::{
        components::toasts::TOASTER_ID, contexts::toasts::leptos_version::Toaster,
        icons::leptos_version::CloseIcon,
    };
    use leptos::{either::Either, prelude::*};

    use super::ToastType;

    #[derive(PartialEq, Clone)]
    pub struct ToastProps {
        pub toast_type: ToastType,
        pub title: String,
        pub description: String,
        pub verbose: Option<String>,
    }

    #[component]
    pub fn Toaster() -> impl IntoView {
        let toaster = use_context::<RwSignal<Toaster>>().unwrap();
        let toasts = move || toaster.get().toasts;

        let toasts_view = move || {
            toasts()
                .iter()
                .map(|toast| toast_comp(toast.clone()))
                .collect_view()
        };

        view! {
            <div id=TOASTER_ID class="toast">
                { toasts_view }
            </div>
        }
    }

    pub fn toast_comp(props: ToastProps) -> impl IntoView {
        let toast_id = props.title.clone();

        let toaster = use_context::<RwSignal<Toaster>>().unwrap();

        let close = {
            let props_clone = props.clone();
            move |_| {
                toaster.update(|x| x.remove_toast(&props_clone));
            }
        };

        let alert_classes =
            "flex flex-col px-8 py-6 rounded-xl 2xl:w-124 xl:w-112 lg:w-96 md:w-80 w-[97.5%]"
                .to_string()
                + match props.toast_type {
                    ToastType::Default => "",
                    ToastType::Success => "bg-success",
                    ToastType::Error => "bg-error",
                    ToastType::Info => "bg-info",
                };

        match &props.verbose {
            Some(description) => Either::Left(view! {
                <div
                    id=toast_id
                    class=alert_classes
                >
                    <div class="flex w-full justify-end">
                        <button
                            on:click=close
                            class="btn btn-ghost"
                        >
                            <CloseIcon class="h-8 w-8".to_string() />
                        </button>
                    </div>
                    <div
                        class="flex flex-col items-start space-y-4"
                    >
                        <span class="font-bold text-xl">{props.title.clone()}</span>
                        <span>{props.description}</span>
                    </div>
                    <div class="collapse mt-8">
                        <input type="checkbox" />
                        <div class="collapse-title p-0 m-0 flex flex-row justify-start">
                        <button class="btn">{"More..."}</button>
                        </div>
                        <div class="collapse-content">
                            <div class="font-mono w-full overflow-auto">
                                {description.clone()}
                            </div>
                        </div>
                    </div>
                </div>
            }),
            None => Either::Right(view! {
                <div id=toast_id class=alert_classes>
                    <div class="flex flex-col items-start space-y-4 mt-2">
                        <span class="font-bold text-xl">{props.title.clone()}</span>
                        <span>{props.description}</span>
                    </div>
                    <div class="flex w-full justify-end mt-4">
                        <button on:click={close} class="btn btn-outline">
                            {"Close"}
                        </button>
                    </div>
                </div>
            }),
        }
    }
}

#[function_component(Toaster)]
pub fn toaster() -> Html {
    let toasts = use_toaster().state().toasts;

    let mut toasts_html: Vec<Html> = Vec::new();

    toasts.iter().for_each(|toast| {
        toasts_html.push(html!(<Toast ..toast.clone()/>));
    });

    html! {
        <div id={TOASTER_ID} class="toast">
            { toasts_html }
        </div>
    }
}

#[derive(PartialEq, Clone)]
pub enum ToastType {
    Default,
    Error,
    Success,
    Info,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ToastProps {
    pub toast_type: ToastType,
    pub title: AttrValue,
    pub description: AttrValue,
    pub verbose: Option<String>,
}

impl From<UbiquityError> for ToastProps {
    fn from(error: UbiquityError) -> Self {
        let toast_type = ToastType::Error;
        let title: AttrValue = error.title.into();
        let description: AttrValue = error.human_description.into();
        let verbose = error.verbose_description;

        ToastProps {
            toast_type,
            title,
            description,
            verbose,
        }
    }
}

#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let toast_id = &props.title;

    let toaster = use_toaster();

    let close = Callback::from({
        let props_clone = props.clone();
        move |_| {
            toaster.remove_toast(props_clone.clone());
        }
    });

    let mut alert_classes = classes!(
        "flex",
        "flex-col",
        "px-8",
        "py-6",
        "rounded-xl",
        "2xl:w-124",
        "xl:w-112",
        "lg:w-96",
        "md:w-80",
        "w-[97.5%]"
    );
    match props.toast_type {
        ToastType::Default => alert_classes.push(""),
        ToastType::Success => alert_classes.push("bg-success"),
        ToastType::Error => alert_classes.push("bg-error"),
        ToastType::Info => alert_classes.push("bg-info"),
    }

    match &props.verbose {
        Some(description) => html! {
            <div id={toast_id} class={alert_classes}>
                <div class="flex w-full justify-end">
                    <button onclick={close} class="btn btn-ghost">
                        <CloseIcon classes={"h-8 w-8"} />
                    </button>
                </div>
                <div class="flex flex-col items-start space-y-4">
                    <span class="font-bold text-xl">{&props.title}</span>
                    <span>{&props.description}</span>
                </div>
                <div class="collapse mt-8">
                    <input type="checkbox" />
                    <div class="collapse-title p-0 m-0 flex flex-row justify-start">
                    <button class="btn">{"More..."}</button>
                    </div>
                    <div class="collapse-content">
                        <div class="font-mono w-full overflow-auto">
                            {description.as_str()}
                        </div>
                    </div>
                </div>
            </div>
        },
        None => html! {
            <div id={toast_id} class={alert_classes}>
                <div class="flex flex-col items-start space-y-4 mt-2">
                    <span class="font-bold text-xl">{&props.title}</span>
                    <span>{&props.description}</span>
                </div>
                <div class="flex w-full justify-end mt-4">
                    <button onclick={close} class="btn btn-outline">
                        {"Close"}
                    </button>
                </div>
            </div>
        },
    }
}
