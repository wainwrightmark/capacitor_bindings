use crate::{error::Error, plugin_listener_handle::PluginListenerHandle};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{future::Future, sync::Arc};
use wasm_bindgen::{prelude::Closure, JsValue};
use wasm_bindgen_futures::JsFuture;

/// An error that is returned by some capacitor functions.
#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct InnerError {
    pub message: String,
}

/// An exception thrown by a javascript function
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct JsException {
    pub message: String,
}

/// Runs a function that takes a unit value and returns a unit result.
pub async fn run_unit_unit<Fut: Future<Output = Result<(), JsValue>>, F: Fn() -> Fut>(
    f: F,
) -> Result<(), Error> {
    Ok(f().await?)
}

/// Runs a function that takes a unit value and returns a typed result.
pub async fn run_unit_value<
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

/// Runs a function that takes a typed value and returns a unit result.
pub async fn run_value_unit<
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

/// Runs a function that takes a typed value and returns a typed result.
pub async fn run_value_value<
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

pub async fn listen_async<T: serde::de::DeserializeOwned, F: Fn(T) + 'static>(
    func: F,
    name: &'static str,
    add_listener: impl Fn(&str, &Closure<dyn Fn(JsValue)>) -> JsValue,
) -> Result<PluginListenerHandle, Error> {
    let func2 = move |js_value: JsValue| {
        let schema: T = serde_wasm_bindgen::from_value(js_value)
            .map_err(|e| Error::deserializing::<T>(e))
            .unwrap(); //deserialize should always succeed assuming I have done everything else right
        func(schema)
    };
    let closure = Arc::new(Closure::new(func2));

    let js_value = add_listener(name, closure.as_ref());

    let promise = Promise::resolve(&js_value);
    let future = JsFuture::from(promise);

    let handle = future.await?;

    Ok(PluginListenerHandle::new(closure, handle))
}
