use serde::{Deserialize, Serialize};

mod error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmittedMarkdown {
    pub current: usize,
    pub len: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmittedConfig {
    pub theme_notification: bool,
    pub slide_notification: bool,
    pub live_config_reload: bool,
}

impl From<GlobalConfig> for EmittedConfig {
    fn from(
        GlobalConfig {
            theme_notification,
            slide_notification,
            live_config_reload,
            ..
        }: GlobalConfig,
    ) -> Self {
        Self {
            theme_notification,
            slide_notification,
            live_config_reload,
        }
    }
}

impl EmittedMarkdown {
    pub fn new(current: usize, len: usize) -> Self {
        Self { current, len }
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

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct GlobalConfig {
    pub default_theme: String,
    pub default_font_size: FontSize,
    pub theme_notification: bool,
    pub slide_notification: bool,
    pub live_config_reload: bool,
}
#[cfg(feature = "server")]
pub mod server_only {
    use std::path::Path;

    use crate::GlobalConfig;
    const CONFIG_NAME: &str = ".iquity_config.toml";

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
        }
    }
}
