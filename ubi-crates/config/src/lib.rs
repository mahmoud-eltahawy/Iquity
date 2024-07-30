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
    Space = 32,
    #[serde(rename = "'")]
    Quote = 39,
    #[serde(rename = ",")]
    Comma = 44,
    #[serde(rename = "-")]
    Minus = 189,
    #[serde(rename = ".")]
    Period = 46,
    #[serde(rename = "/")]
    Slash = 191,
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
    #[serde(rename = ";")]
    Semicolon = 59,
    #[serde(rename = "=")]
    Equal = 187,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    #[serde(rename = "[")]
    BracketLeft = 91,
    #[serde(rename = "\\")]
    Backslash = 92,
    #[serde(rename = "]")]
    BracketRight = 93,
    #[serde(rename = "`")]
    Backquote = 96,
    Escape = 256,
    Enter = 257,
    Tab = 258,
    Backspace = 259,
    Insert = 260,
    Delete = 261,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
    PageUp = 266,
    PageDown = 267,
    Home = 268,
    End = 269,
    CapsLock = 280,
    ScrollLock = 281,
    NumLock = 282,
    PrintScreen = 283,
    Pause = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    NumPad0 = 320,
    NumPad1 = 321,
    NumPad2 = 322,
    NumPad3 = 323,
    NumPad4 = 324,
    NumPad5 = 325,
    NumPad6 = 326,
    NumPad7 = 327,
    NumPad8 = 328,
    NumPad9 = 329,
    NumpadDecimal = 330,
    NumpadDivide = 331,
    NumpadMultiply = 332,
    NumpadSubtract = 333,
    NumpadAdd = 334,
    NumpadEnter = 335,
    NumpadEqual = 336,
    ShiftLeft = 340,
    ControlLeft = 341,
    AltLeft = 342,
    MetaLeft = 343,
    ShiftRight = 344,
    ControlRight = 345,
    AltRight = 346,
    MetaRight = 347,
    ContextMenu = 348,
}

impl From<u16> for KeyName {
    fn from(value: u16) -> Self {
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
