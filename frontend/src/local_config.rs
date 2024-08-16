use std::{cell::RefCell, collections::HashMap, rc::Rc};

use config::{Action, EmittedConfig, InitConfig, KeyName};
use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct Config {
    pub font_size: RwSignal<u8>,
    pub theme_notification: Rc<RefCell<bool>>,
    pub live_config_reload: Rc<RefCell<bool>>,
    pub keys: Rc<RefCell<HashMap<KeyName, Action>>>,
    pub keys_help: RwSignal<String>,
    pub port: Rc<RefCell<u16>>,
}

impl Config {
    pub fn set(
        &self,
        InitConfig {
            conf,
            keys_help,
            port,
        }: InitConfig,
    ) {
        if conf.default_font_size != self.font_size.get_untracked() {
            self.font_size.set(conf.default_font_size);
        }
        *self.theme_notification.borrow_mut() = conf.theme_notification;
        *self.keys.borrow_mut() = conf.keys.to_map();
        self.keys_help.set(keys_help);
        *self.port.borrow_mut() = port;
    }

    pub fn update(
        &self,
        EmittedConfig {
            theme_notification,
            live_config_reload,
            keys,
            keys_help,
            port,
        }: EmittedConfig,
    ) {
        *self.theme_notification.borrow_mut() = theme_notification;
        *self.live_config_reload.borrow_mut() = live_config_reload;
        *self.keys.borrow_mut() = keys.to_map();
        self.keys_help.set(keys_help);
        *self.port.borrow_mut() = port;
    }

    pub fn increase_font_size(&self) {
        self.font_size.update(|x| *x += 1);
    }

    pub fn decrease_font_size(&self) {
        self.font_size
            .update(|x| *x = x.checked_sub(1).unwrap_or(16));
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            font_size: RwSignal::new(16),
            theme_notification: Rc::new(RefCell::new(true)),
            live_config_reload: Rc::new(RefCell::new(true)),
            keys: Rc::new(RefCell::new(HashMap::new())),
            keys_help: RwSignal::new("".to_string()),
            port: Rc::new(RefCell::new(80)),
        }
    }
}
