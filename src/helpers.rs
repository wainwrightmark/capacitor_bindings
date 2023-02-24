use std::future::Future;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{ JsValue};

#[derive(Debug)]
pub enum Error{
    JsException{message: String},
    SerializeError{typename: &'static str, error: serde_wasm_bindgen::Error},
    DeserializeError{typename: &'static str, error: serde_wasm_bindgen::Error}
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Error::JsException { message } => write!(f, "Js Exception: {message}"),
            Error::SerializeError { typename, error } => write!(f, "Error Serializing {typename} to JsValue: {error}"),
            Error::DeserializeError { typename, error } => write!(f, "Error Deserializing JsValue to {typename}: {error}"),
        }
    }
}

impl std::error::Error for Error{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self{
            Error::JsException { .. } => None,
            Error::SerializeError { error,.. } => error.source(),
            Error::DeserializeError {  error,.. } => error.source(),
        }
    }
}

impl Error{
    pub fn deserializing<O: serde::de::DeserializeOwned>(error: serde_wasm_bindgen::Error)-> Self{
        let typename = std::any::type_name::<O>();
        Self::DeserializeError { typename, error }
    }

    pub fn serializing<I: serde::Serialize>(error: serde_wasm_bindgen::Error)-> Self{
        let typename = std::any::type_name::<I>();
        Self::DeserializeError { typename, error }
    }
}

impl From<JsValue> for Error{
    fn from(value: JsValue) -> Self {
        if let Ok(exception) = serde_wasm_bindgen::from_value::<JsException> (value.clone()){
            Error::JsException { message: exception.message }
        }else{
            Error::JsException { message: format!("{value:?}") }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JsException{
    pub message: String
}

pub async fn run_unit_unit<Fut: Future<Output = Result<(), JsValue> >, F: Fn() -> Fut>(f: F)-> Result<(), Error> {
    Ok(f().await?)
}

pub async fn run_unit_value<
    'de,
    O: serde::de::DeserializeOwned,
    Fut: Future<Output = Result<JsValue,JsValue> >,
    F: Fn() -> Fut,
>(
    f: F,
) -> Result<O, Error> {
    let js_value = f().await?;
    let o: O = serde_wasm_bindgen::from_value(js_value).map_err(|e| Error::deserializing::<O>(e))?;
    Ok(o)
}

pub async fn run_value_unit<
    I: serde::Serialize,
    Fut: Future<Output = Result<(),JsValue>>,
    F: Fn(JsValue) -> Fut,
>(
    i: impl Into<I>,
    f: F,
) -> Result<(), Error> {
    let i = i.into();
    let js_value: JsValue = serde_wasm_bindgen::to_value(&i).map_err(|e| Error::serializing::<I>(e))?;
    Ok(f(js_value).await?)
}

pub async fn run_value_value<
    'de,
    O: serde::de::DeserializeOwned,
    I: serde::Serialize,
    Fut: Future<Output = Result<JsValue,JsValue>>,
    F: Fn(JsValue) -> Fut,
>(
    i: impl Into<I>,
    f: F,
) -> Result<O, Error> {
    let i = i.into();
    let js_input_value: JsValue = serde_wasm_bindgen::to_value(&i).map_err(|e| Error::serializing::<I>(e))?;
    let js_output_value = f(js_input_value).await?;
    let o: O = serde_wasm_bindgen::from_value(js_output_value).map_err(|e| Error::deserializing::<O>(e))?;
    Ok(o)
}
