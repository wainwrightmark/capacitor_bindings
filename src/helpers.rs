use js_sys::{Function, Promise};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use wasm_bindgen_futures::JsFuture;
use std::{future::Future, sync::Arc};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

/// An error used as a field on some functions
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InnerError{
    pub message: String
}

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

impl From<InnerError> for Error{
    fn from(value: InnerError) -> Self {
        Self::JsException { message: value.message }
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JsException {
    pub message: String,
}

pub (crate) async fn run_unit_unit<Fut: Future<Output = Result<(), JsValue>>, F: Fn() -> Fut>(
    f: F,
) -> Result<(), Error> {
    Ok(f().await?)
}

pub (crate) async fn run_unit_value<
    'de,
    O: serde::de::DeserializeOwned,
    Fut: Future<Output = Result<JsValue, JsValue>>,
    F: Fn() -> Fut,
>(
    f: F,
) -> Result<O, Error> {
    let js_value = f().await?;
    let o: O =
        serde_wasm_bindgen::from_value(js_value).map_err(|e| Error::deserializing::<O>(e))?;
    Ok(o)
}

pub (crate) async fn run_value_unit<
    I: serde::Serialize,
    Fut: Future<Output = Result<(), JsValue>>,
    F: Fn(JsValue) -> Fut,
>(
    i: impl Into<I>,
    f: F,
) -> Result<(), Error> {
    let i = i.into();
    let js_value: JsValue =
        serde_wasm_bindgen::to_value(&i).map_err(|e| Error::serializing::<I>(e))?;
    Ok(f(js_value).await?)
}

pub (crate) async fn run_value_value<
    'de,
    O: serde::de::DeserializeOwned,
    I: serde::Serialize,
    Fut: Future<Output = Result<JsValue, JsValue>>,
    F: Fn(JsValue) -> Fut,
>(
    i: impl Into<I>,
    f: F,
) -> Result<O, Error> {
    let i = i.into();
    let js_input_value: JsValue =
        serde_wasm_bindgen::to_value(&i).map_err(|e| Error::serializing::<I>(e))?;
    let js_output_value = f(js_input_value).await?;
    let o: O = serde_wasm_bindgen::from_value(js_output_value)
        .map_err(|e| Error::deserializing::<O>(e))?;
    Ok(o)
}

pub (crate) async fn listen_async<T: serde::de::DeserializeOwned, F: Fn(T) + 'static>(
    func: F,
    name: &'static str,
    add_listener: impl Fn(&str, &Closure<dyn Fn(JsValue)>) -> JsValue,
) -> Result<PluginListenerHandle, Error> {
    let func2 = move |js_value: JsValue| {
        let schema: T = serde_wasm_bindgen::from_value(js_value)
            .map_err(|e| Error::deserializing::<T>(e))
            .unwrap();
        func(schema)
    };
    let closure = Arc::new(Closure::new(func2));

    let js_value = add_listener(name, closure.as_ref());

    let promise = Promise::resolve(&js_value);
    let future = JsFuture::from(promise);

    let handle = future.await?;

    Ok(PluginListenerHandle::new(closure, handle))
}




/// A handle for a listener
#[derive(Debug, Clone)]
pub struct PluginListenerHandle {
    _closure: Arc<Closure<dyn Fn(JsValue)>>,
    handle: JsValue,
}

impl PluginListenerHandle {
    pub (crate)  fn new(closure: Arc<Closure<dyn Fn(JsValue)>>, handle: JsValue) -> Self {

        Self { handle, _closure: closure }
    }
}


impl PartialEq for PluginListenerHandle {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl PluginListenerHandle {
    /// Remove this listener
    pub async fn remove_async(self) -> Result<(), Error> {
        let remove = js_sys::Reflect::get(&self.handle, &JsValue::from_str("remove"))?;
        let remove_function = remove
            .dyn_ref::<Function>()
            .ok_or(Error::NotAFunction { name: "remove" })?;

        let result = remove_function.call0(&self.handle)?;

        let promise = js_sys::Promise::resolve(&result);

        wasm_bindgen_futures::JsFuture::from(promise).await?;
        Ok(())
    }
}
