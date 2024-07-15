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
    let classes = "flex flex-col justify-between max-w-[calc(100svw)] print:hidden min-h-screen";

    div()
        .attr("data-theme", theme)
        .attr("class", classes)
        .on(ev::touchstart, on_touch_start)
        .on(ev::touchend, on_touch_end)
        .child(children())
}
