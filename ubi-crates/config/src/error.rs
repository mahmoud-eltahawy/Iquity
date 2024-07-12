use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    Io(#[from] io::Error),

    // #[error("Error deserialising RON to string.")]
    // RonToStringError(#[from] ron::de::Error),
    #[error("Error deserialising RON to string.")]
    RonToString(#[from] ron::de::SpannedError),

    #[error("An error occured with RON.")]
    Ron(#[from] ron::Error),
}
