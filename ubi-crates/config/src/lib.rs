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
    ) -> Self {
        Self {
            theme_notification,
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
    Number(u8),
    Letter(char),
    F(u8),
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    Esc,
    Tab,
    Space,
    Enter,
    Equal,
    Minus,
    Slash,
    Backslash,
    Backquote,
    Backspace,
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<String> for KeyName {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let key = if let Some(num) = value
            .chars()
            .nth("Digit".len())
            .and_then(|x| x.to_string().parse::<u8>().ok())
        {
            Self::Number(num)
        } else if let Ok(num) = value[1..].parse::<u8>() {
            Self::F(num)
        } else if value.starts_with("Key") && value.len() == 4 {
            Self::Letter(value.chars().nth(3).unwrap().to_ascii_lowercase())
        } else {
            match value.as_str() {
                "AltLeft" => Self::AltLeft,
                "AltRight" => Self::AltRight,
                "ShiftLeft" => Self::ShiftLeft,
                "ShiftRight" => Self::ShiftRight,
                "ControlLeft" => Self::ControlLeft,
                "ControlRight" => Self::ControlRight,
                "Escape" => Self::Esc,
                "Tab" => Self::Tab,
                "Space" => Self::Space,
                "Equal" => Self::Equal,
                "Minus" => Self::Minus,
                "Enter" => Self::Enter,
                "Slash" => Self::Slash,
                "Backslash" => Self::Backslash,
                "Backquote" => Self::Backquote,
                "Backspace" => Self::Backspace,
                "ArrowLeft" => Self::Left,
                "ArrowRight" => Self::Right,
                "ArrowUp" => Self::Up,
                "ArrowDown" => Self::Down,
                _ => return Err("unlisted key name".to_string()),
            }
        };
        Ok(key)
    }
}

impl Display for KeyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            KeyName::Number(number) => format!("Digit{}", number),
            KeyName::Letter(letter) => format!("Key{}", letter.to_ascii_uppercase()),
            KeyName::F(num) => format!("F{num}"),
            KeyName::AltLeft => "AltLeft".to_string(),
            KeyName::AltRight => "AltRight".to_string(),
            KeyName::ShiftLeft => "ShiftLeft".to_string(),
            KeyName::ShiftRight => "ShiftRight".to_string(),
            KeyName::ControlLeft => "ControlLeft".to_string(),
            KeyName::ControlRight => "ControlRight".to_string(),
            KeyName::Esc => "Escape".to_string(),
            KeyName::Tab => "Tab".to_string(),
            KeyName::Space => "Space".to_string(),
            KeyName::Equal => "Equal".to_string(),
            KeyName::Minus => "Minus".to_string(),
            KeyName::Enter => "Enter".to_string(),
            KeyName::Slash => "Slash".to_string(),
            KeyName::Backslash => "/".to_string(),
            KeyName::Backquote => "Backquote".to_string(),
            KeyName::Backspace => "Backspace".to_string(),
            KeyName::Left => "Left".to_string(),
            KeyName::Right => "Right".to_string(),
            KeyName::Up => "Up".to_string(),
            KeyName::Down => "Down".to_string(),
        };
        write!(f, "{}", name)
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
            print: KeyName::Letter('p'),
            next_theme: KeyName::Letter('j'),
            prev_theme: KeyName::Letter('k'),
            next_slide: KeyName::Letter('l'),
            prev_slide: KeyName::Letter('h'),
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
