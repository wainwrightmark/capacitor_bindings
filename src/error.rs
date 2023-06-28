use wasm_bindgen::JsValue;

use crate::helpers::{InnerError, JsException};

/// An error that can happen when calling a capacitor function
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    JsException {
        message: String,
    },
    NotAFunction {
        name: &'static str,
    },
    SerializeError {
        typename: &'static str,
        error: serde_wasm_bindgen::Error,
    },
    DeserializeError {
        typename: &'static str,
        error: serde_wasm_bindgen::Error,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JsException { message } => write!(f, "Js Exception: {message}"),
            Error::SerializeError { typename, error } => {
                write!(f, "Error Serializing {typename} to JsValue: {error}")
            }
            Error::DeserializeError { typename, error } => {
                write!(f, "Error Deserializing JsValue to {typename}: {error}")
            }
            Error::NotAFunction { name } => write!(f, "Not a function: {name}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::JsException { .. } => None,
            Error::NotAFunction { .. } => None,
            Error::SerializeError { error, .. } => error.source(),
            Error::DeserializeError { error, .. } => error.source(),
        }
    }
}

impl Error {
    pub fn deserializing<O: serde::de::DeserializeOwned>(error: serde_wasm_bindgen::Error) -> Self {
        let typename = std::any::type_name::<O>();
        Self::DeserializeError { typename, error }
    }

    pub fn serializing<I: serde::Serialize>(error: serde_wasm_bindgen::Error) -> Self {
        let typename = std::any::type_name::<I>();
        Self::DeserializeError { typename, error }
    }
}

impl From<InnerError> for Error {
    fn from(value: InnerError) -> Self {
        Self::JsException {
            message: value.message,
        }
    }
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        if let Ok(exception) = serde_wasm_bindgen::from_value::<JsException>(value.clone()) {
            Error::JsException {
                message: exception.message,
            }
        } else {
            Error::JsException {
                message: format!("{value:?}"),
            }
        }
    }
}
