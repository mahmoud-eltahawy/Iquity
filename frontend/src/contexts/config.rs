use config::Config;
use error::UbiquityError;
use gloo::storage::{LocalStorage, Storage};
use leptos::reactive_graph::signal::RwSignal;
use std::ops::Deref;
use yew::prelude::*;

static CONFIG_STORAGE_KEY: &str = "config";

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConfigContext {
    inner: UseStateHandle<Config>,
}

impl ConfigContext {
    pub fn new(inner: UseStateHandle<Config>) -> Self {
        Self { inner }
    }

    pub fn state(&self) -> Config {
        self.inner.current().clone()
    }

    pub fn set(&self, config: Config) -> Result<(), UbiquityError> {
        self.inner.set(config.clone());
        self.save(config)?;
        Ok(())
    }

    pub fn set_theme(&self, theme: String) -> Result<(), UbiquityError> {
        let mut new_config = self.state();
        new_config.theme = theme;
        self.set(new_config)?;
        Ok(())
    }

    pub fn save(&self, config: Config) -> Result<(), UbiquityError> {
        let value = config.to_string()?;
        LocalStorage::set(CONFIG_STORAGE_KEY, value)?;
        Ok(())
    }

    pub fn load_from_storage() -> Result<Config, UbiquityError> {
        let ron_str: String = LocalStorage::get(CONFIG_STORAGE_KEY)?;
        let config: Config = Config::from_ron_str(&ron_str)?;
        Ok(config)
    }

    pub fn increase_font_size(&self) -> Result<(), UbiquityError> {
        let mut new_config = self.state();
        new_config.increase_font_size();
        self.set(new_config)?;
        Ok(())
    }

    pub fn decrease_font_size(&self) -> Result<(), UbiquityError> {
        let mut new_config = self.state();
        new_config.decrease_font_size();
        self.set(new_config)?;
        Ok(())
    }

    pub fn increase_preview_font_size(&self) -> Result<(), UbiquityError> {
        let mut new_config = self.state();
        new_config.increase_preview_font_size();
        self.set(new_config)?;
        Ok(())
    }

    pub fn decrease_preview_font_size(&self) -> Result<(), UbiquityError> {
        let mut new_config = self.state();
        new_config.decrease_preview_font_size();
        self.set(new_config)?;
        Ok(())
    }
}

impl Deref for ConfigContext {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        self.inner.current()
    }
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ConfigProviderProps {
    pub children: Children,
}

#[function_component]
pub(crate) fn ConfigProvider(props: &ConfigProviderProps) -> Html {
    let config = ConfigContext::load_from_storage().unwrap_or_default();

    let config_state = use_state(|| config);
    let config_context = ConfigContext::new(config_state);

    html! {
        <ContextProvider<ConfigContext> context={config_context}>
            {props.children.clone()}
        </ContextProvider<ConfigContext>>
    }
}

pub fn config_provider() -> RwSignal<Config> {
    let config = ConfigContext::load_from_storage().unwrap_or_default();

    let config_state = leptos::prelude::RwSignal::new(config);

    config_state
}

#[hook]
pub(crate) fn use_config() -> ConfigContext {
    use_context::<ConfigContext>().unwrap()
}

pub const THEMES: &[&str] = &[
    "light",
    "dark",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
    "synthwave",
    "retro",
    "cyberpunk",
    "valentine",
    "halloween",
    "garden",
    "forest",
    "aqua",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "black",
    "luxury",
    "dracula",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "night",
    "coffee",
    "winter",
];
