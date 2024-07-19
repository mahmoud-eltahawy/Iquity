use config::GlobalConfig;
use leptos::prelude::*;

pub const THEMES_SIZE: usize = THEMES.len();
pub const THEMES: &[&str] = &[
    "dracula",
    "synthwave",
    "dark",
    "light",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
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
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "night",
    "coffee",
    "winter",
    "dim",
    "nord",
    "sunset",
];

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub theme_index: RwSignal<usize>,
    pub font_size: RwSignal<String>,
}

impl Config {
    pub fn increase_font_size(self) {
        self.font_size.update(|x| {
            *x = match x.as_str() {
                "prose-sm" => "prose-base".to_string(),
                "prose-base" => "prose-lg".to_string(),
                "prose-lg" => "prose-xl".to_string(),
                "prose-xl" => "prose-2xl".to_string(),
                "prose-2xl" => "prose-2xl".to_string(),
                _ => "prose-base".to_string(),
            }
        });
    }

    pub fn decrease_font_size(self) {
        self.font_size.update(|x| {
            *x = match x.as_str() {
                "prose-2xl" => "prose-xl".to_string(),
                "prose-xl" => "prose-lg".to_string(),
                "prose-lg" => "prose-base".to_string(),
                "prose-base" => "prose-sm".to_string(),
                "prose-sm" => "prose-sm".to_string(),
                _ => "prose-base".to_string(),
            }
        });
    }

    pub fn next_theme(self) {
        self.theme_index.update(|x| *x += 1);
    }

    pub fn prev_theme(self) {
        self.theme_index
            .update(|x| *x = x.checked_sub(1).unwrap_or(THEMES_SIZE - 1));
    }
}

impl From<GlobalConfig> for Config {
    fn from(
        GlobalConfig {
            theme_index,
            font_size,
        }: GlobalConfig,
    ) -> Self {
        Self {
            theme_index: RwSignal::new(theme_index),
            font_size: RwSignal::new(match font_size {
                config::FontSize::VerySmall => "prose-sm".to_string(),
                config::FontSize::Small => "prose-base".to_string(),
                config::FontSize::Middle => "prose-lg".to_string(),
                config::FontSize::Big => "prose-xl".to_string(),
                config::FontSize::VeryBig => "prose-2xl".to_string(),
            }),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from(GlobalConfig::default())
    }
}
