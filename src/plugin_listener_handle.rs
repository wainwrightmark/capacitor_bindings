use std::sync::Arc;

use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

use crate::error::Error;

/// A handle for a listener.
/// If this is dropped, the callback will not work, so either store it somewhere for removal later using `remove_async` or call `leak`.
#[derive(Clone, Debug)]
#[must_use = "Handle must not be dropped without calling `remove_async`"]
pub struct PluginListenerHandle {
    _closure: Arc<Closure<dyn Fn(JsValue)>>,
    handle: JsValue,
}

impl PluginListenerHandle {
    pub(crate) fn new(closure: Arc<Closure<dyn Fn(JsValue)>>, handle: JsValue) -> Self {
        Self {
            handle,
            _closure: closure,
        }
    }

    /// Leak this listener so it will never be dropped
    pub fn leak(self) {
        Box::leak(Box::new(self));
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
