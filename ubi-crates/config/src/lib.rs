use ::error::UbiquityError;
use dirs::{config_dir, data_dir};
use ron::ser::PrettyConfig;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

mod error;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Config {
    pub theme: String,
    pub md_input_font_size: String,
    pub md_preview_font_size: String,
    pub data_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "synthwave".to_string(),
            data_path: data_dir(),
            md_input_font_size: String::from("text-base"),
            md_preview_font_size: String::from("prose-base"),
        }
    }
}

impl Config {
    pub fn increase_font_size(&mut self) {
        self.md_input_font_size = match self.md_input_font_size.as_str() {
            "text-xs" => "text-sm".to_string(),
            "text-sm" => "text-base".to_string(),
            "text-base" => "text-lg".to_string(),
            "text-lg" => "text-xl".to_string(),
            "text-xl" => "text-2xl".to_string(),
            "text-2xl" => "text-3xl".to_string(),
            "text-3xl" => "text-4xl".to_string(),
            "text-4xl" => "text-5xl".to_string(),
            "text-5xl" => "text-6xl".to_string(),
            "text-6xl" => "text-7xl".to_string(),
            "text-7xl" => "text-8xl".to_string(),
            "text-8xl" => "text-9xl".to_string(),
            "text-9xl" => "text-9xl".to_string(),
            _ => "text-base".to_string(),
        };
    }

    pub fn decrease_font_size(&mut self) {
        self.md_input_font_size = match self.md_input_font_size.as_str() {
            "text-xs" => "text-xs".to_string(),
            "text-sm" => "text-xs".to_string(),
            "text-base" => "text-sm".to_string(),
            "text-lg" => "text-base".to_string(),
            "text-xl" => "text-lg".to_string(),
            "text-2xl" => "text-xl".to_string(),
            "text-3xl" => "text-2xl".to_string(),
            "text-4xl" => "text-3xl".to_string(),
            "text-5xl" => "text-4xl".to_string(),
            "text-6xl" => "text-5xl".to_string(),
            "text-7xl" => "text-6xl".to_string(),
            "text-8xl" => "text-7xl".to_string(),
            "text-9xl" => "text-8xl".to_string(),
            _ => "text-base".to_string(),
        };
    }

    pub fn increase_preview_font_size(&mut self) {
        self.md_preview_font_size = match self.md_preview_font_size.as_str() {
            "prose-sm" => "prose-base".to_string(),
            "prose-base" => "prose-lg".to_string(),
            "prose-lg" => "prose-xl".to_string(),
            "prose-xl" => "prose-2xl".to_string(),
            "prose-2xl" => "prose-2xl".to_string(),
            _ => "prose-base".to_string(),
        };
    }

    pub fn decrease_preview_font_size(&mut self) {
        self.md_preview_font_size = match self.md_preview_font_size.as_str() {
            "prose-sm" => "prose-sm".to_string(),
            "prose-base" => "prose-sm".to_string(),
            "prose-lg" => "prose-base".to_string(),
            "prose-xl" => "prose-lg".to_string(),
            "prose-2xl" => "prose-xl".to_string(),
            _ => "prose-base".to_string(),
        };
    }

    pub fn save(&self) -> Result<(), UbiquityError> {
        let pretty_ron_config = PrettyConfig::default();
        let ron_string = ron::ser::to_string_pretty(self, pretty_ron_config)?;
        write_config_file(&ron_string)?;
        Ok(())
    }

    pub fn from_ron_str(ron_str: &str) -> Result<Self, UbiquityError> {
        let config: Config = ron::from_str(ron_str)?;
        Ok(config)
    }

    pub fn to_string(&self) -> Result<String, UbiquityError> {
        let pretty_ron_config = PrettyConfig::default();
        Ok(ron::ser::to_string_pretty(self, pretty_ron_config)?)
    }

    pub fn load(&mut self) -> Result<(), UbiquityError> {
        let config = read_config_file()?;
        *self = config;
        Ok(())
    }

    pub fn init() -> Result<(), UbiquityError> {
        let config = Self::default();
        config.save()?;
        Ok(())
    }

    pub fn current(&self) -> &Self {
        self
    }
}

pub fn read_config_file() -> Result<Config, UbiquityError> {
    let path = get_config_file()?;
    let config_str = fs::read_to_string(path)?;
    let config: Config = ron::from_str(&config_str)?;
    Ok(config)
}

pub fn write_config_file(ron_string: &str) -> Result<(), UbiquityError> {
    let path = get_config_file()?;
    fs::write(path, ron_string)?;
    Ok(())
}

pub fn get_config_file() -> Result<PathBuf, UbiquityError> {
    let mut path = get_config_folder()?;
    path.push("config.ron");
    match path.exists() {
        true => Ok(path),
        false => {
            fs::write(path.clone(), "")?;
            Ok(path)
        }
    }
}

pub fn get_config_folder() -> Result<PathBuf, UbiquityError> {
    match config_dir() {
        Some(mut path) => {
            path.push("ubiquity/");
            fs::create_dir(&path)?;
            Ok(path)
        }
        None => Err(UbiquityError::no_config_folder()),
    }
}
