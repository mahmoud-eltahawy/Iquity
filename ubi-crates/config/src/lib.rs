use std::fmt::Display;

use serde::{Deserialize, Serialize};

mod error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmittedMarkdown<T: ToString> {
    pub current: usize,
    pub len: usize,
    pub content: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmittedConfig {
    pub theme_notification: bool,
    pub slide_notification: bool,
    pub live_config_reload: bool,
    pub keys: Keys,
    pub keys_help: String,
}

impl EmittedConfig {
    pub fn new(
        GlobalConfig {
            theme_notification,
            slide_notification,
            live_config_reload,
            keys,
            ..
        }: GlobalConfig,
        keys_help: String,
    ) -> Self {
        Self {
            theme_notification,
            slide_notification,
            live_config_reload,
            keys,
            keys_help,
        }
    }
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
pub const CONFIG_EVENT: &str = "config";

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FontSize {
    VerySmall,
    Small,
    Middle,
    Big,
    VeryBig,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GlobalConfig {
    pub default_theme: String,
    pub default_font_size: FontSize,
    pub theme_notification: bool,
    pub slide_notification: bool,
    pub live_config_reload: bool,
    pub keys: Keys,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Keys {
    pub print: String,
    pub next_theme: String,
    pub prev_theme: String,
    pub next_slide: String,
    pub prev_slide: String,
    pub increase_fontsize: String,
    pub decrease_fontsize: String,
    pub help: String,
}

impl Display for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            print,
            next_theme,
            prev_theme,
            next_slide,
            prev_slide,
            increase_fontsize,
            decrease_fontsize,
            help,
        } = self;
        write!(
            f,
            r#"
|         **key**          |       **Action**        |
|:------------------------:|:-----------------------:|
|       **{print}**        |        __print__        |
|     **{next_theme}**     |      __next theme__     |
|     **{prev_theme}**     |    __previous theme__   |
|     **{next_slide}**     |      __next slide__     |
|     **{prev_slide}**     |     __previous slide__  |
| **{increase_fontsize}**  |   __increase fontsize__ |
| **{decrease_fontsize}**  |   __decrease fontsize__ |
|       **{help}**         |         __help__        |
|       **Esc**            |   __hide this message__ |
"#
        )
    }
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            print: "KeyP".to_string(),
            next_theme: "KeyJ".to_string(),
            prev_theme: "KeyK".to_string(),
            next_slide: "KeyL".to_string(),
            prev_slide: "KeyH".to_string(),
            increase_fontsize: "Minus".to_string(),
            decrease_fontsize: "Equal".to_string(),
            help: "Slash".to_string(),
        }
    }
}

#[cfg(feature = "server")]
pub mod server_only {
    use std::path::{Path, PathBuf};

    use crate::GlobalConfig;
    const CONFIG_NAME: &str = ".iquity/config.toml";

    impl GlobalConfig {
        pub fn config_path() -> Option<std::path::PathBuf> {
            dirs::home_dir().map(|mut x| {
                x.push(CONFIG_NAME);
                x
            })
        }
        fn to_toml(&self) -> Result<String, toml::ser::Error> {
            toml::to_string(self)
        }
        fn from_toml(text: &str) -> Result<Self, toml::de::Error> {
            toml::from_str(text)
        }

        pub async fn get<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
            let path = PathBuf::from(path.as_ref());
            let parent = path.parent().unwrap();
            if !parent.exists() {
                let _ = tokio::fs::create_dir(parent).await;
            }
            let text = tokio::fs::read_to_string(&path).await;
            let config = match text {
                Ok(text) => GlobalConfig::from_toml(&text)?,
                Err(_) => {
                    let gb = GlobalConfig::default();
                    let text = gb.to_toml()?;
                    tokio::fs::write(&path, text).await?;
                    gb
                }
            };
            Ok(config)
        }
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            default_theme: "dracula".to_string(),
            default_font_size: FontSize::Small,
            theme_notification: true,
            slide_notification: true,
            live_config_reload: true,
            keys: Keys::default(),
        }
    }
}
