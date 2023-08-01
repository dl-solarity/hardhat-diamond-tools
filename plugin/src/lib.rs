use wasm_bindgen::prelude::*;

mod actions;
mod node_bindings;

/// The main entrypoint of the plugin. This file will be [`required`]
/// in [`hardhat.config.js`] and here all tasks will be created.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // task("names", "Test functionality").set_action(&names_action);

    Ok(())
}
