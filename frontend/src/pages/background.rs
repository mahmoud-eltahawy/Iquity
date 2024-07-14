use crate::contexts::config::use_config;
use gloo::utils::document;
use log::debug;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub mod leptos_version {
    use config::Config;
    use gloo::utils::document;
    use leptos::{
        ev::{self, TouchEvent},
        html::div,
        prelude::*,
    };
    use wasm_bindgen::{JsCast, UnwrapThrowExt};
    use web_sys::HtmlInputElement;

    #[component]
    pub fn Background(children: Children) -> impl IntoView {
        let conf = use_context::<RwSignal<Config>>().unwrap_throw();
        let theme = move || conf.get().theme;

        let touch_start_x = RwSignal::new(0);
        let touch_start_y = RwSignal::new(0);
        let touch_end_x = RwSignal::new(0);
        let touch_end_y = RwSignal::new(0);

        let on_touch_start = move |touch_event: TouchEvent| {
            let user_touch = touch_event.changed_touches().item(0).unwrap_throw();
            touch_start_x.set(user_touch.client_x());
            touch_start_y.set(user_touch.client_y());
        };

        let on_touch_end = move |touch_event: TouchEvent| {
            let user_touch = touch_event.changed_touches().item(0).unwrap_throw();
            touch_end_x.set(user_touch.client_x());
            touch_end_y.set(user_touch.client_y());

            let start_x = touch_start_x.get();
            let end_x = touch_end_x.get();
            let start_y = touch_start_y.get();
            let end_y = touch_end_y.get();

            let drawer: HtmlInputElement = document()
                .get_element_by_id("drawer-input")
                .unwrap()
                .dyn_into()
                .unwrap();
            if end_x > (start_x + 50) && (start_y <= end_y + 50 && start_y >= end_y - 50) {
                drawer.set_checked(true);
            }
        };

        // let mut classes = classes!("flex", "flex-col", "justify-between", "min-h-[calc(100svh)]", "max-h-[calc(100svh)]", "min-w-[calc(100svw)]", "max-w-[calc(100svw)]");
        let classes =
            "flex flex-col justify-between max-w-[calc(100svw)] print:hidden min-h-screen";

        div()
            .attr("data-theme", theme)
            .attr("class", classes)
            .on(ev::touchstart, on_touch_start)
            .on(ev::touchend, on_touch_end)
            .child(children())
    }
}

#[derive(Properties, PartialEq)]
pub struct BackgroundProps {
    pub children: Children,
}

#[function_component(Background)]
pub fn background(props: &BackgroundProps) -> Html {
    let theme = use_config().state().theme;

    let touch_start_x = use_mut_ref(|| 0);
    let touch_start_y = use_mut_ref(|| 0);
    let touch_end_x = use_mut_ref(|| 0);
    let touch_end_y = use_mut_ref(|| 0);

    let touch_start_x_clone = touch_start_x.clone();
    let touch_start_y_clone = touch_start_y.clone();
    let touch_end_x_clone = touch_end_x.clone();
    let touch_end_y_clone = touch_end_y.clone();

    let on_touch_start = Callback::from(move |touch_event: TouchEvent| {
        debug!("On touch");
        let user_touch = touch_event.changed_touches().item(0).unwrap_throw();
        touch_start_x.replace(user_touch.client_x());
        touch_start_y.replace(user_touch.client_y());
    });

    let on_touch_end = Callback::from(move |touch_event: TouchEvent| {
        debug!("End touch");
        let user_touch = touch_event.changed_touches().item(0).unwrap_throw();
        touch_end_x_clone.replace(user_touch.client_x());
        touch_end_y_clone.replace(user_touch.client_y());

        let start_x = *touch_start_x_clone.borrow();
        let end_x = *touch_end_x_clone.borrow();
        let start_y = *touch_start_y_clone.borrow();
        let end_y = *touch_end_y_clone.borrow();

        let drawer: HtmlInputElement = document()
            .get_element_by_id("drawer-input")
            .unwrap()
            .dyn_into()
            .unwrap();
        if end_x > (start_x + 50) && (start_y <= end_y + 50 && start_y >= end_y - 50) {
            drawer.set_checked(true);
        }
    });

    // let mut classes = classes!("flex", "flex-col", "justify-between", "min-h-[calc(100svh)]", "max-h-[calc(100svh)]", "min-w-[calc(100svw)]", "max-w-[calc(100svw)]");
    let classes = classes!(
        "flex",
        "flex-col",
        "justify-between",
        "max-w-[calc(100svw)]",
        "print:hidden",
        "min-h-screen"
    );

    html! {
        <div ontouchstart={on_touch_start} ontouchend={on_touch_end} data-theme={theme} class={classes}>
            { props.children.clone() }
        </div>
    }
}
