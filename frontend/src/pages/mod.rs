use yew_router::prelude::*;

pub mod background;
pub mod home;
pub mod settings;

#[derive(Clone, PartialEq, Routable)]
pub enum Page {
    #[at("/")]
    Home,
    #[at("/settings")]
    Settings,
}
