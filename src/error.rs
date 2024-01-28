use std::io;

use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Python exception: {0:?}")]
    Python(String),

    #[error("Cannot resolve resource {0}")]
    Resolve(String),

    #[error("{0} is not a directory")]
    NotADir(String),

    #[error(transparent)]
    IO(#[from] io::Error),

    #[error("JSON error: {0:?}")]
    Json(#[from] serde_json::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
