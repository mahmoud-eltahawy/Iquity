use yew::prelude::*;

mod leptos_version {
    use leptos::prelude::*;
    #[component]
    pub fn DividerYAxis() -> impl IntoView {
        view! {
            <div class="divider divider-horizontal mx-1"/>
        }
    }

    #[component]
    pub fn DividerXAxis() -> impl IntoView {
        view! {
            <div class="divider divider-vertical"/>
        }
    }
}

#[function_component(DividerYAxis)]
pub fn divider_y_axis() -> Html {
    html! {
        <div class="divider divider-horizontal mx-1"/>
    }
}

#[function_component(DividerXAxis)]
pub fn divider_x_axis() -> Html {
    html! {
        <div class="divider divider-vertical"/>
    }
}
