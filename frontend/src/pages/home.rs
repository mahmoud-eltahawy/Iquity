use crate::components::dual_view::DualView;
use crate::components::header::Header;
use crate::components::modals::Modals;
use crate::components::pdf::Pdf;
use crate::components::toasts::Toaster;
use crate::pages::background::Background;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <Background>
            <Header />
            <div class="h-[calc(100vh-4rem)] flex flex-col content-center align-center items-center justify-center">
                <DualView />
            </div>
            <Modals />
            <Toaster />
        </Background>
        <Pdf />
        </>
    }
}
