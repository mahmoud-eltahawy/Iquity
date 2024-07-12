use std::{
    fmt::{self, Debug, Display, Formatter},
    io,
    ops::Range,
    string::FromUtf8Error,
};

use gloo::storage::errors::StorageError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UbiquityError {
    pub title: String,
    pub human_description: String,
    pub verbose_description: Option<String>,
}

impl Display for UbiquityError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.verbose_description {
            Some(verbose_description) => write!(
                f,
                "An error has occured in Ubiquity:/n/n{}/n/n{}/n/n{}",
                self.title, self.human_description, verbose_description
            ),
            None => write!(
                f,
                "An error has occured in Ubiquity:/n/n{}/n/n{}",
                self.title, self.human_description
            ),
        }
    }
}

impl Default for UbiquityError {
    fn default() -> Self {
        let title: String = String::from("Unknown Error");
        let human_description: String = String::from("An error has occured.");
        let verbose_description: Option<String> = Some(String::from("Error code: 492068617665206e6f20636c75652077686174206a7573742068617070656e65642e204920686f6e6573746c7920646f6e2774207265616c6c792063617265206569746865722e20537570706f727420746869732070726f6a65637420616e642049202a6d696768742a206669782069742e"));
        Self {
            title,
            human_description,
            verbose_description,
        }
    }
}

impl From<io::Error> for UbiquityError {
    fn from(io_error: io::Error) -> Self {
        let title = String::from("I/O Error");
        let human_description = io_error.kind().to_string();
        let verbose_description = None;
        Self {
            title,
            human_description,
            verbose_description,
        }
    }
}

impl From<tauri_sys::error::Error> for UbiquityError {
    fn from(tauri_error: tauri_sys::error::Error) -> Self {
        let mut tauri_error_object_string = tauri_error.to_string();

        let range = Range { start: 0, end: 27 };
        tauri_error_object_string.replace_range(range, "");
        tauri_error_object_string.pop();
        tauri_error_object_string.pop();

        // "JS Binding: JsValue(Object({"human_description":"There was no save path selected.","title":"Save Error","verbose_description":null}))"

        let ubi_error: UbiquityError = serde_json::from_str(&tauri_error_object_string).unwrap();

        let title = ubi_error.title;
        let human_description = ubi_error.human_description;
        let verbose_description = ubi_error.verbose_description;
        Self {
            title,
            human_description,
            verbose_description,
        }
    }
}

impl From<StorageError> for UbiquityError {
    fn from(storage_error: StorageError) -> Self {
        let title = String::from("Browser Storage Error");
        let human_description = storage_error.to_string();
        let verbose_description = None;
        Self {
            title,
            human_description,
            verbose_description,
        }
    }
}

impl From<ron::Error> for UbiquityError {
    fn from(ron_error: ron::Error) -> Self {
        let title = String::from("RON Error");
        let human_description = String::from("An error occured with RON.");
        let verbose_description = Some(ron_error.to_string());
        Self {
            title,
            human_description,
            verbose_description,
        }
    }
}

impl From<FromUtf8Error> for UbiquityError {
    fn from(utf8_error: FromUtf8Error) -> Self {
        let title = String::from("File Read Error");
        let human_description = String::from("Could not convert the selected file to a string.");
        let verbose_description = Some(utf8_error.to_string());
        Self {
            title,
            human_description,
            verbose_description,
        }
    }
}

impl From<ron::error::SpannedError> for UbiquityError {
    fn from(ron_error: ron::error::SpannedError) -> Self {
        let title = String::from("RON Error");
        let human_description = String::from("An error occured with RON.");
        let verbose_description = Some(ron_error.to_string());
        Self {
            title,
            human_description,
            verbose_description,
        }
    }
}

impl UbiquityError {
    pub fn no_config_folder() -> Self {
        let title = String::from("Config Folder Error");
        let human_description = String::from(
            "Ubiquity could not find your operating system's default configuration folder.",
        );
        let verbose_description = None;
        Self {
            title,
            human_description,
            verbose_description,
        }
    }

    pub fn no_save_path_selected() -> Self {
        let title = String::from("Save Error");
        let human_description = String::from("There was no save path selected.");
        let verbose_description = None;
        Self {
            title,
            human_description,
            verbose_description,
        }
    }

    pub fn no_file_selected() -> Self {
        let title = String::from("Open Error");
        let human_description = String::from("There was no file selected for opening.");
        let verbose_description = None;
        Self {
            title,
            human_description,
            verbose_description,
        }
    }

    pub fn mdtg(err: String) -> Self {
        let title = String::from("Markdown Table Error");
        let human_description = String::from("There was an error generating your markdown table.");
        let verbose_description = Some(err);
        Self {
            title,
            human_description,
            verbose_description,
        }
    }
}
