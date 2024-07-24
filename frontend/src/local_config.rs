use std::{cell::RefCell, rc::Rc};

use config::{EmittedConfig, GlobalConfig, Keys};
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

#[derive(Debug, Clone)]
pub struct Config {
    pub theme_index: RwSignal<usize>,
    pub font_size: RwSignal<String>,
    pub theme_notification: Rc<RefCell<bool>>,
    pub slide_notification: Rc<RefCell<bool>>,
    pub live_config_reload: Rc<RefCell<bool>>,
    pub keys: Rc<RefCell<Keys>>,
}

impl Config {
    pub fn set(&self, conf: GlobalConfig) {
        let theme_index = THEMES
            .iter()
            .enumerate()
            .find(|(_, x)| x.to_string() == conf.default_theme)
            .map(|(i, _)| i)
            .unwrap_or(0);
        let font_size = match conf.default_font_size {
            config::FontSize::VerySmall => "prose-sm".to_string(),
            config::FontSize::Small => "prose-base".to_string(),
            config::FontSize::Middle => "prose-lg".to_string(),
            config::FontSize::Big => "prose-xl".to_string(),
            config::FontSize::VeryBig => "prose-2xl".to_string(),
        };
        if theme_index != self.theme_index.get_untracked() {
            self.theme_index.set(theme_index);
        }
        if font_size != self.font_size.get_untracked() {
            self.font_size.set(font_size);
        }
        if conf.theme_notification != *self.theme_notification.borrow() {
            *self.theme_notification.borrow_mut() = conf.theme_notification;
        }
        if conf.slide_notification != *self.slide_notification.borrow() {
            *self.slide_notification.borrow_mut() = conf.slide_notification;
        }
        *self.keys.borrow_mut() = conf.keys;
    }

    pub fn update(
        &self,
        EmittedConfig {
            theme_notification,
            slide_notification,
            live_config_reload,
        }: EmittedConfig,
    ) {
        *self.theme_notification.borrow_mut() = theme_notification;
        *self.slide_notification.borrow_mut() = slide_notification;
        *self.live_config_reload.borrow_mut() = live_config_reload;
    }

    pub fn increase_font_size(&self) {
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

    pub fn decrease_font_size(&self) {
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

    pub fn next_theme(&self) {
        self.theme_index.update(|x| *x += 1);
    }

    pub fn prev_theme(&self) {
        self.theme_index
            .update(|x| *x = x.checked_sub(1).unwrap_or(THEMES_SIZE - 1));
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme_index: RwSignal::new(0),
            font_size: RwSignal::new("prose-base".to_string()),
            theme_notification: Rc::new(RefCell::new(true)),
            slide_notification: Rc::new(RefCell::new(true)),
            live_config_reload: Rc::new(RefCell::new(true)),
            keys: Rc::new(RefCell::new(Keys::default())),
        }
    }
}
