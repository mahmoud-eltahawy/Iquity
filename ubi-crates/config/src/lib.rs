use serde::{Deserialize, Serialize};

mod error;

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

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Config {
    pub theme_index: usize,
    pub font_size: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme_index: 0,
            font_size: String::from("prose-base"),
        }
    }
}

impl Config {
    pub fn theme(&self) -> &'static str {
        THEMES[self.theme_index % THEMES_SIZE]
    }
    pub fn increase_font_size(&mut self) {
        self.font_size = match self.font_size.as_str() {
            "prose-sm" => "prose-base".to_string(),
            "prose-base" => "prose-lg".to_string(),
            "prose-lg" => "prose-xl".to_string(),
            "prose-xl" => "prose-2xl".to_string(),
            "prose-2xl" => "prose-2xl".to_string(),
            _ => "prose-base".to_string(),
        };
    }

    pub fn decrease_font_size(&mut self) {
        self.font_size = match self.font_size.as_str() {
            "prose-sm" => "prose-sm".to_string(),
            "prose-base" => "prose-sm".to_string(),
            "prose-lg" => "prose-base".to_string(),
            "prose-xl" => "prose-lg".to_string(),
            "prose-2xl" => "prose-xl".to_string(),
            _ => "prose-base".to_string(),
        };
    }

    pub fn next_theme(&mut self) {
        self.theme_index += 1;
    }

    pub fn prev_theme(&mut self) {
        self.theme_index = self.theme_index.checked_sub(1).unwrap_or(THEMES_SIZE - 1);
    }
}
