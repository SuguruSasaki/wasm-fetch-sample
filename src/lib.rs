use std::borrow::Borrow;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    let msg = format!("Hello {}", name);
    alert(&msg);
}

#[wasm_bindgen]
pub async fn request(path: String) -> js_sys::Promise {
    let res = reqwest::get(path).await;
    if res.is_err() {
        alert("error!!");
        return js_sys::Promise::reject(&"error".into());
    }
    let data = res.unwrap().text().await;
    
    match data {
        Ok(s) => js_sys::Promise::resolve(&s.into()),
        Err(_) => js_sys::Promise::reject(&"fetch error".into())
    }
}
