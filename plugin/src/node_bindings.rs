use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// `console.log` for rust.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

/// `fs` module of `nodejs` for usage in rust.
#[wasm_bindgen(module = "node:fs")]
extern "C" {
    #[wasm_bindgen(js_name = writeFileSync, catch)]
    pub fn write_file_sync(path: &str, data: &str) -> Result<(), JsValue>;
}
