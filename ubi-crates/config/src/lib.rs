use serde::{Deserialize, Serialize};

mod error;

pub const CONTENT_EVENT: &str = "content";

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub enum FontSize {
    VerySmall,
    Small,
    Middle,
    Big,
    VeryBig,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct GlobalConfig {
    pub theme_index: usize,
    pub font_size: FontSize,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            theme_index: 0,
            font_size: FontSize::Small,
        }
    }
}
