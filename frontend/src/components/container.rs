use leptos::{html::div, prelude::*};

pub fn container(children: Children) -> impl IntoView {
    div()
        .attr("class", "my-auto border border-base-content rounded-xl p-4 pb-8 2xl:w-[65%] xl:w-[72.5%] lg:w-[80%] md:w-[87.5%] sm-[95%] w-[98%] h-[calc(100dvh-7.25rem)]")
        .child(children())
}

pub fn half_width_container(children: impl IntoView) -> impl IntoView {
    div()
        .attr("class", "w-[48%] flex-none h-[calc(100vh-8.25rem)] border border-base-content rounded-xl pt-4 pb-6 px-8")
        .child(children)
}
