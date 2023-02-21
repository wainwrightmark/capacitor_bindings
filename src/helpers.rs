use std::future::Future;

use wasm_bindgen::JsValue;

pub async fn run_unit_unit<Fut: Future<Output = ()>, F: Fn() -> Fut>(f: F) {
    f().await
}

pub async fn run_unit_value<
    'de,
    O: serde::de::DeserializeOwned,
    Fut: Future<Output = JsValue>,
    F: Fn() -> Fut,
>(
    f: F,
) -> O {
    let js_value = f().await;
    let o: O = serde_wasm_bindgen::from_value(js_value).expect(std::any::type_name::<O>());
    o
}

pub async fn run_value_unit<
    I: serde::Serialize,
    Fut: Future<Output = ()>,
    F: Fn(JsValue) -> Fut,
>(
    i: impl Into<I>,
    f: F,
) -> () {
    let i = i.into();
    let js_value: JsValue = serde_wasm_bindgen::to_value(&i).expect(std::any::type_name::<I>());
    f(js_value).await
}

pub async fn run_value_value<
    'de,
    O: serde::de::DeserializeOwned,
    I: serde::Serialize,
    Fut: Future<Output = JsValue>,
    F: Fn(JsValue) -> Fut,
>(
    i: impl Into<I>,
    f: F,
) -> O {
    let i = i.into();
    let js_input_value: JsValue =
        serde_wasm_bindgen::to_value(&i).expect(std::any::type_name::<I>());
    let js_output_value = f(js_input_value).await;
    let o: O = serde_wasm_bindgen::from_value(js_output_value).expect(std::any::type_name::<O>());
    o
}
