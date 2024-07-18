use std::collections::HashMap;

use error::UbiquityError;
use gloo::storage::{LocalStorage, Storage};
use leptos::prelude::*;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Markdown {
    pub text: String,
    pub key: Option<String>,
}

impl Markdown {
    pub fn new() -> Self {
        let key = None;
        let text = String::from("");
        Self { text, key }
    }
    pub fn from(text: String, key: Option<String>) -> Self {
        Self { text, key }
    }
    pub fn load_latest_from_storage() -> Option<Markdown> {
        let vec = Self::read_all_markdown_keys();
        vec.last()
            .map(|key| Markdown::load_from_storage(key.clone()))
    }

    pub fn read_all_markdown_keys() -> Vec<String> {
        let storage_vec: HashMap<String, Value> = LocalStorage::get_all().unwrap();
        let mut markdown_keys_vec: Vec<String> = Vec::new();
        storage_vec.iter().for_each(|storage_item| {
            if !storage_item.0.eq("config") && !storage_item.0.eq("ubiquity_about.md") {
                let key_str = storage_item.0.clone();
                markdown_keys_vec.push(key_str);
            }
        });
        markdown_keys_vec
    }

    pub fn load_from_storage(key: String) -> Markdown {
        let key_str = key.to_string();
        let text_str: String = LocalStorage::get(key_str).unwrap();
        let key = Some(key);
        Markdown {
            text: text_str,
            key,
        }
    }

    pub fn update_text(&mut self, text: String) -> Result<(), UbiquityError> {
        self.text = text;
        self.save_to_browser_storage()?;
        Ok(())
    }

    pub fn save_to_browser_storage(&self) -> Result<(), UbiquityError> {
        let key = self.key.as_ref().expect("No key.");
        let key_str = key.as_str();
        let text_str = self.text.as_str();
        LocalStorage::set(key_str, text_str)?;
        Ok(())
    }
}
