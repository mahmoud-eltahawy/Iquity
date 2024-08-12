use std::{collections::HashMap, fmt::Display, mem::transmute};

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
    pub live_config_reload: bool,
    pub keys: Keys,
    pub keys_help: String,
    pub port: u16,
}

impl EmittedConfig {
    pub fn new(
        GlobalConfig {
            theme_notification,
            live_config_reload,
            keys,
            ..
        }: GlobalConfig,
        keys_help: String,
        port: u16,
    ) -> Self {
        Self {
            theme_notification,
            live_config_reload,
            keys,
            keys_help,
            port,
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

#[derive(Debug, Clone)]
pub enum Action {
    Print,
    NextTheme,
    PrevTheme,
    NextSlide,
    PrevSlide,
    IncreaseFontsize,
    DecreaseFontsize,
    Help,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct InitConfig {
    pub conf: GlobalConfig,
    pub keys_help: String,
    pub port: u16,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GlobalConfig {
    pub default_theme: String,
    pub default_font_size: FontSize,
    pub theme_notification: bool,
    pub live_config_reload: bool,
    pub keys: Keys,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum KeyName {
    //line 1
    Esc = 27,
    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    // line 2
    #[serde(rename = "`")]
    Backquote = 192,
    #[serde(rename = "0")]
    Digit0 = 48,
    #[serde(rename = "1")]
    Digit1 = 49,
    #[serde(rename = "2")]
    Digit2 = 50,
    #[serde(rename = "3")]
    Digit3 = 51,
    #[serde(rename = "4")]
    Digit4 = 52,
    #[serde(rename = "5")]
    Digit5 = 53,
    #[serde(rename = "6")]
    Digit6 = 54,
    #[serde(rename = "7")]
    Digit7 = 55,
    #[serde(rename = "8")]
    Digit8 = 56,
    #[serde(rename = "9")]
    Digit9 = 57,
    #[serde(rename = "-")]
    Minus = 189,
    #[serde(rename = "=")]
    Equal = 187,
    Backspace = 8,
    // line 3
    Tab = 9,
    Q = 81,
    W = 87,
    E = 69,
    R = 82,
    T = 84,
    Y = 89,
    U = 85,
    I = 73,
    O = 79,
    P = 80,
    #[serde(rename = "[")]
    BracketLeft = 219,
    #[serde(rename = "]")]
    BracketRight = 221,
    #[serde(rename = "\\")]
    Backslash = 220,
    //line 4
    A = 65,
    S = 83,
    D = 68,
    F = 70,
    G = 71,
    H = 72,
    J = 74,
    K = 75,
    L = 76,
    #[serde(rename = ";")]
    Semicolon = 186,
    #[serde(rename = "'")]
    Quote = 222,
    Enter = 13,
    //line 5
    Shift = 16,
    Z = 90,
    X = 88,
    C = 67,
    V = 86,
    B = 66,
    N = 78,
    M = 77,
    #[serde(rename = ",")]
    Comma = 188,
    #[serde(rename = ".")]
    Period = 190,
    #[serde(rename = "/")]
    Slash = 191,
    #[serde(rename = "`")]
    //line 6
    Alt = 18,
    Control = 17,
    Space = 32,
    //arrows
    Up = 38,
    Down = 40,
    Right = 39,
    Left = 37,
}

impl From<u8> for KeyName {
    fn from(value: u8) -> Self {
        unsafe { transmute(value) }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Keys {
    pub print: KeyName,
    pub next_theme: KeyName,
    pub prev_theme: KeyName,
    pub next_slide: KeyName,
    pub prev_slide: KeyName,
    pub increase_fontsize: KeyName,
    pub decrease_fontsize: KeyName,
    pub help: KeyName,
}

impl Keys {
    pub fn to_map(self) -> HashMap<KeyName, Action> {
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
        HashMap::from([
            (print, Action::Print),
            (next_theme, Action::NextTheme),
            (prev_theme, Action::PrevTheme),
            (next_slide, Action::NextSlide),
            (prev_slide, Action::PrevSlide),
            (increase_fontsize, Action::IncreaseFontsize),
            (decrease_fontsize, Action::DecreaseFontsize),
            (help, Action::Help),
        ])
    }
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
|       **{print:?}**        |        __print__        |
|     **{next_theme:?}**     |      __next theme__     |
|     **{prev_theme:?}**     |    __previous theme__   |
|     **{next_slide:?}**     |      __next slide__     |
|     **{prev_slide:?}**     |     __previous slide__  |
| **{increase_fontsize:?}**  |   __increase fontsize__ |
| **{decrease_fontsize:?}**  |   __decrease fontsize__ |
|       **{help:?}**         |         __help__        |
|       **Esc**            |   __hide this message__ |
"#
        )
    }
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            print: KeyName::P,
            next_theme: KeyName::J,
            prev_theme: KeyName::K,
            next_slide: KeyName::L,
            prev_slide: KeyName::H,
            increase_fontsize: KeyName::Equal,
            decrease_fontsize: KeyName::Minus,
            help: KeyName::Slash,
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
            live_config_reload: true,
            keys: Keys::default(),
        }
    }
}
