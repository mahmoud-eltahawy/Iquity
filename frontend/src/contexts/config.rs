use config::Config;
use error::UbiquityError;
use gloo::storage::{LocalStorage, Storage};
use leptos::reactive_graph::signal::RwSignal;

static CONFIG_STORAGE_KEY: &str = "config";

pub fn load_from_storage() -> Result<Config, UbiquityError> {
    let ron_str: String = LocalStorage::get(CONFIG_STORAGE_KEY)?;
    let config: Config = Config::from_ron_str(&ron_str)?;
    Ok(config)
}

pub fn config_provider() -> RwSignal<Config> {
    let config = load_from_storage().unwrap_or_default();

    let config_state = RwSignal::new(config);

    config_state
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
