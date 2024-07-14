use error::UbiquityError;
use gloo::storage::LocalStorage;
use gloo::storage::Storage;
use md::{DOCS_KEY, DOCS_STR};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::ops::Deref;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Markdown {
    pub text: AttrValue,
    pub key: Option<AttrValue>,
}

pub mod leptos_version {
    use std::collections::HashMap;

    use error::UbiquityError;
    use gloo::storage::{LocalStorage, Storage};
    use leptos::prelude::*;
    use md::{DOCS_KEY, DOCS_STR};
    use serde_json::Value;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Markdown {
        pub text: String,
        pub key: Option<String>,
    }

    impl Default for Markdown {
        fn default() -> Self {
            let text = String::from(DOCS_STR);
            let key = Some(String::from(DOCS_KEY));
            Self { text, key }
        }
    }

    impl Markdown {
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

        pub fn update(&mut self, md: Markdown) -> Result<(), UbiquityError> {
            *self = md;
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

    pub fn markdown_provider() -> RwSignal<Markdown> {
        let markdown = Markdown::load_latest_from_storage().unwrap_or_default();

        let markdown_state = RwSignal::new(markdown);

        markdown_state
    }
}

impl Default for Markdown {
    fn default() -> Self {
        let text = AttrValue::from(DOCS_STR);
        let key = Some(AttrValue::from(DOCS_KEY));
        Self { text, key }
    }
}

impl Markdown {
    pub fn from(text: AttrValue, key: Option<AttrValue>) -> Self {
        Self { text, key }
    }

    pub fn current(&self) -> &Self {
        self
    }

    pub fn read_all_markdown_keys() -> Vec<AttrValue> {
        let storage_vec: HashMap<String, Value> = LocalStorage::get_all().unwrap();
        let mut markdown_keys_vec: Vec<AttrValue> = Vec::new();
        storage_vec.iter().for_each(|storage_item| {
            if !storage_item.0.eq("config") && !storage_item.0.eq("ubiquity_about.md") {
                let key_str = storage_item.0.clone();
                let key = AttrValue::from(key_str);
                markdown_keys_vec.push(key);
            }
        });
        markdown_keys_vec
    }

    pub fn load_from_storage(key: AttrValue) -> Markdown {
        let key_str = key.to_string();
        let text_str: String = LocalStorage::get(key_str).unwrap();
        let text = AttrValue::from(text_str);
        let key = Some(key);
        Markdown { text, key }
    }

    pub fn load_latest_from_storage() -> Option<Markdown> {
        let vec = Self::read_all_markdown_keys();
        vec.last()
            .map(|key| Markdown::load_from_storage(key.clone()))
    }

    pub fn save_to_browser_storage(&self) -> Result<(), UbiquityError> {
        let key = self.key.as_ref().expect("No key.");
        let key_str = key.as_str();
        let text_str = self.text.as_str();
        LocalStorage::set(key_str, text_str)?;
        Ok(())
    }

    pub fn remove_from_browser_storage(&self) {
        let key = self.key.as_ref().expect("No key.");
        let key_str = key.as_str();
        LocalStorage::delete(key_str);
    }
}

impl MarkdownContext {
    pub fn new(inner: UseStateHandle<Markdown>) -> Self {
        Self { inner }
    }

    pub fn update_markdown(&self, md: Markdown) -> Result<(), UbiquityError> {
        self.inner.set(md);
        self.save_to_browser_storage()?;
        Ok(())
    }

    pub fn set_markdown(&self, md: Markdown) -> Result<(), UbiquityError> {
        self.inner.set(md);
        self.save_to_browser_storage()?;
        Ok(())
    }

    pub fn update_key(&self, key: AttrValue) {
        let text = self.text.clone();
        let key = Some(key);
        let new_md = Markdown::from(text, key);

        self.inner.remove_from_browser_storage();
        self.inner.set(new_md);
    }

    pub fn add_markdown(&self, markdown: Markdown) -> Result<(), UbiquityError> {
        markdown.save_to_browser_storage()?;
        self.inner.set(markdown);
        Ok(())
    }

    pub fn state(&self) -> Markdown {
        self.inner.current().clone()
    }
}

impl Deref for MarkdownContext {
    type Target = Markdown;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for MarkdownContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MarkdownContext {
    inner: UseStateHandle<Markdown>,
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct MarkdownProviderProps {
    pub children: Children,
}

#[function_component]
pub(crate) fn MarkdownProvider(props: &MarkdownProviderProps) -> Html {
    let markdown = Markdown::load_latest_from_storage().unwrap_or_default();

    let markdown_state = use_state(|| markdown);
    let markdown_context = MarkdownContext::new(markdown_state);

    html! {
        <ContextProvider<MarkdownContext> context={markdown_context}>
            {props.children.clone()}
        </ContextProvider<MarkdownContext>>
    }
}

#[hook]
pub(crate) fn use_markdown() -> MarkdownContext {
    use_context::<MarkdownContext>().unwrap()
}

#[derive(Deserialize)]
pub struct BrowserStorageItem {
    pub keys: Vec<String>,
}
