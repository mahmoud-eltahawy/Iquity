use serde::{Deserialize, Serialize};

mod error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmittedMarkdown<T: ToString> {
    pub current: usize,
    pub len: usize,
    pub content: T,
}

impl<T> EmittedMarkdown<T>
where
    T: ToString,
{
    pub fn new(current: usize, len: usize, content: T) -> Self {
        Self {
            current,
            len,
            content,
        }
    }
}

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
