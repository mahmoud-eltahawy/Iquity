// use crate::components::pdf::Pdf;
use crate::components::dual_view::DualView;
use crate::pages::background::leptos_version::Background;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Background>
            <div class="h-[calc(100vh-4rem)] flex flex-col content-center align-center items-center justify-center">
                <DualView />
            </div>
        </Background>
        // <Pdf />
    }
}
