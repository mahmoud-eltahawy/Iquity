use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct ContainerProps {
    pub children: Children,
}

pub mod leptos_version {
    use leptos::{html::div, prelude::*};

    #[component]
    pub fn Container(children: Children) -> impl IntoView {
        div()
            .attr("class", "my-auto border border-base-content rounded-xl p-4 pb-8 2xl:w-[65%] xl:w-[72.5%] lg:w-[80%] md:w-[87.5%] sm-[95%] w-[98%] h-[calc(100dvh-7.25rem)]")
            .child(children())
    }
    #[component]
    pub fn HalfWidthContainer(children: Children) -> impl IntoView {
        div()
            .attr("class", "w-[48%] flex-none h-[calc(100vh-8.25rem)] border border-base-content rounded-xl pt-4 pb-6 px-8")
            .child(children())
    }
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let container_classes = classes!(
        "my-auto",
        "border",
        "border-base-content",
        "rounded-xl",
        "p-4",
        "pb-8",
        "2xl:w-[65%]",
        "xl:w-[72.5%]",
        "lg:w-[80%]",
        "md:w-[87.5%]",
        "sm-[95%]",
        "w-[98%]",
        "h-[calc(100dvh-7.25rem)]"
    );

    html! {
        <div class={container_classes}>
            { props.children.clone() }
        </div>
    }
}

#[function_component(HalfWidthContainer)]
pub fn half_width_container(props: &ContainerProps) -> Html {
    html! {
        <div class="w-[48%] flex-none h-[calc(100vh-8.25rem)] border border-base-content rounded-xl pt-4 pb-6 px-8">
            { props.children.clone() }
        </div>
    }
}
