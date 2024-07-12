use crate::components::modals::Modals;
use crate::components::toasts::Toaster;
use crate::components::{drawer::Drawer, welcome_hero::WelcomeHero};
use crate::pages::background::Background;
use yew::prelude::*;

#[function_component(Welcome)]
pub fn welcome() -> Html {
    html! {
        <Drawer>
            <Background>
                <WelcomeHero />
            </Background>
            <Modals />
            <Toaster />
        </Drawer>
    }
}
