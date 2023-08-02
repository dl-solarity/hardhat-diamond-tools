use wasm_bindgen::prelude::wasm_bindgen;

/// `console.log` for rust.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub mod fs {
    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct MkdirOptions {
        pub recursive: bool,
        pub mode: u32,
    }

    impl Default for MkdirOptions {
        fn default() -> Self {
            Self {
                recursive: Default::default(),
                mode: 0o777,
            }
        }
    }

    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    /// `fs` module of `nodejs` for usage in rust.
    #[wasm_bindgen(module = "node:fs")]
    extern "C" {
        #[wasm_bindgen(js_name = writeFileSync, catch)]
        pub fn write_file_sync(path: &str, data: &str) -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "node:fs/promises")]
    extern "C" {
        #[wasm_bindgen(js_name = mkdir, catch)]
        pub async fn mkdir(path: &str, options: MkdirOptions) -> Result<(), JsValue>;
    }
}
