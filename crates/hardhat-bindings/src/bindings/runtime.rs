//! The Hardhat Runtime Environment bindings for Rust.

use super::artifacts::Artifacts;
use wasm_bindgen::prelude::*;

/// The [`HardhatRuntimeEnvironment`] bindings for Rust.
///
/// [`HardhatRuntimeEnvironment`]: https://github.com/NomicFoundation/hardhat/blob/main/packages/hardhat-core/src/types/runtime.ts#L197C18-L197C43
#[wasm_bindgen(module = "hardhat")]
extern "C" {
    /// The Hardhat Runtime Environment.
    pub type HardhatRuntimeEnvironment;

    /// The Hardhat Runtime Environment.
    #[wasm_bindgen(method, getter)]
    pub fn artifacts(this: &HardhatRuntimeEnvironment) -> Artifacts;
}
