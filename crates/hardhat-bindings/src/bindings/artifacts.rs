
use wasm_bindgen::prelude::*;

/// The [`Artifacts`] bindings for Rust.
///
/// [`Artifacts`]: https://github.com/NomicFoundation/hardhat/blob/main/packages/hardhat-core/src/types/artifacts.ts#L1
#[wasm_bindgen(module = "hardhat/types")]
extern "C" {
    pub type Artifacts;

    /// Get all fully qualified names of contracts in artifacts directory.
    ///
    /// Returns the promise with the array of strings of fully qualified names.
    #[wasm_bindgen(method, js_name = "getAllFullyQualifiedNames", catch)]
    pub async fn get_all_fully_qualified_names(this: &Artifacts) -> Result<JsValue, JsValue>;

    /// Get the artifact of the contract with the given fully qualified name.
    ///
    /// Returns the promise with the [`Artifact`] of the contract.
    #[wasm_bindgen(method, js_name = "readArtifactSync", catch)]
    pub fn read_artifact_sync(this: &Artifacts, name: &str) -> Result<Artifact, JsValue>;
}

/// The [`Artifact`] bindings for Rust.
///
/// [`Artifact`]: https://github.com/NomicFoundation/hardhat/blob/main/packages/hardhat-core/src/types/artifacts.ts#L139
#[wasm_bindgen(module = "hardhat/types")]
extern "C" {
    pub type Artifact;

    /// Name of the contract as it was used in the Solidity source code.
    #[wasm_bindgen(method, getter, js_name = "contractName")]
    pub fn contract_name(this: &Artifact) -> String;

    /// Get the list of ABIs of the contract.
    ///
    /// NOTE: The original interface returns the list of `any` type.
    #[wasm_bindgen(method, getter)]
    pub fn abi(this: &Artifact) -> JsValue;
}
