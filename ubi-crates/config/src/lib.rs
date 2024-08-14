use std::{collections::HashMap, fmt::Display};

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
    Esc,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    // line 2
    #[serde(rename = "`")]
    Backquote,
    #[serde(rename = "0")]
    Digit0,
    #[serde(rename = "1")]
    Digit1,
    #[serde(rename = "2")]
    Digit2,
    #[serde(rename = "3")]
    Digit3,
    #[serde(rename = "4")]
    Digit4,
    #[serde(rename = "5")]
    Digit5,
    #[serde(rename = "6")]
    Digit6,
    #[serde(rename = "7")]
    Digit7,
    #[serde(rename = "8")]
    Digit8,
    #[serde(rename = "9")]
    Digit9,
    #[serde(rename = "-")]
    Minus,
    #[serde(rename = "=")]
    Equal,
    Backspace,
    // line 3
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    #[serde(rename = "[")]
    BracketLeft,
    #[serde(rename = "]")]
    BracketRight,
    #[serde(rename = "\\")]
    Backslash,
    //line 4
    CapsLock,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    #[serde(rename = ";")]
    Semicolon,
    #[serde(rename = "'")]
    Quote,
    Enter,
    //line 5
    Shift,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    #[serde(rename = ",")]
    Comma,
    #[serde(rename = ".")]
    Period,
    #[serde(rename = "/")]
    Slash,
    #[serde(rename = "`")]
    //line 6
    Alt,
    Control,
    Space,
    Menu,
    //arrows
    Up,
    Down,
    Right,
    Left,

    Unrecognized,
}

impl From<&str> for KeyName {
    fn from(value: &str) -> Self {
        match value {
            //row 1
            "Escape" => Self::Esc,
            "F1" => Self::F1,
            "F2" => Self::F2,
            "F3" => Self::F3,
            "F4" => Self::F4,
            "F5" => Self::F5,
            "F6" => Self::F6,
            "F7" => Self::F7,
            "F8" => Self::F8,
            "F9" => Self::F9,
            "F10" => Self::F10,
            "F11" => Self::F11,
            "F12" => Self::F12,
            //row 2
            "Backquote" => Self::Backquote,
            "Digit1" => Self::Digit1,
            "Digit2" => Self::Digit2,
            "Digit3" => Self::Digit3,
            "Digit4" => Self::Digit4,
            "Digit5" => Self::Digit5,
            "Digit6" => Self::Digit6,
            "Digit7" => Self::Digit7,
            "Digit8" => Self::Digit8,
            "Digit9" => Self::Digit9,
            "Digit0" => Self::Digit0,
            "Minus" => Self::Minus,
            "Equal" => Self::Equal,
            "Backspace" => Self::Backspace,
            //row 3
            "Tab" => Self::Tab,
            "KeyQ" => Self::Q,
            "KeyW" => Self::W,
            "KeyE" => Self::E,
            "KeyR" => Self::R,
            "KeyT" => Self::T,
            "KeyY" => Self::Y,
            "KeyU" => Self::U,
            "KeyI" => Self::I,
            "KeyO" => Self::O,
            "KeyP" => Self::P,
            "BracketLeft" => Self::BracketLeft,
            "BracketRight" => Self::BracketRight,
            "Backslash" => Self::Backslash,
            //row 4
            "CapsLock" => Self::CapsLock,
            "KeyA" => Self::A,
            "KeyS" => Self::S,
            "KeyD" => Self::D,
            "KeyF" => Self::F,
            "KeyG" => Self::G,
            "KeyH" => Self::H,
            "KeyJ" => Self::J,
            "KeyK" => Self::K,
            "KeyL" => Self::L,
            "Semicolon" => Self::Semicolon,
            "Quote" => Self::Quote,
            "Enter" => Self::Enter,
            //row 5
            "ShiftLeft" | "ShiftRight" => Self::Shift,
            "KeyZ" => Self::Z,
            "KeyX" => Self::X,
            "KeyC" => Self::C,
            "KeyV" => Self::V,
            "KeyB" => Self::B,
            "KeyN" => Self::N,
            "KeyM" => Self::M,
            "Comma" => Self::Comma,
            "Period" => Self::Period,
            "Slash" => Self::Slash,
            //row 6
            "ControlLeft" | "ControlRight" => Self::Control,
            "AltLeft" | "AltRight" => Self::Alt,
            "Space" => Self::Space,
            "ContextMenu" => Self::Menu,
            //arrows
            "ArrowUp" => Self::Up,
            "ArrowDown" => Self::Down,
            "ArrowRight" => Self::Right,
            "ArrowLeft" => Self::Left,
            _ => Self::Unrecognized,
        }
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
