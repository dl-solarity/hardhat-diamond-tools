//! This module provides binding of Hardhat types to Rust types.

pub mod bindings {
    use js_sys::Promise;
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    use self::hre::HardhatRuntimeEnvironment;

    pub mod hre {
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
    }

    pub mod artifacts {
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
            pub async fn get_all_fully_qualified_names(
                this: &Artifacts,
            ) -> Result<JsValue, JsValue>;

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
    }

    /// The [`ActionType`] bindings for Rust.
    ///
    /// NOTE: First value if the arguments of the task defined by plugin developer, that's why
    /// we use `any` (see [`JsValue`]). The third one `runSuper`.
    ///
    /// [`ActionType`]: https://github.com/NomicFoundation/hardhat/blob/main/packages/hardhat-core/src/types/runtime.ts#L161
    pub type ActionType = dyn Fn(JsValue, HardhatRuntimeEnvironment, JsValue) -> Promise;

    /// The [`ConfigurableTaskDefinition`] bindings for Rust.
    ///
    /// [`ConfigurableTaskDefinition`]: https://github.com/NomicFoundation/hardhat/blob/main/packages/hardhat-core/src/types/runtime.ts#L43C18-L43C44
    #[wasm_bindgen(module = "hardhat/config")]
    extern "C" {
        pub type ConfigurableTaskDefinition;

        #[wasm_bindgen(method, js_name = "setDescription")]
        pub fn set_description(
            this: &ConfigurableTaskDefinition,
            description: &str,
        ) -> ConfigurableTaskDefinition;

        #[wasm_bindgen(method, js_name = "setAction")]
        pub fn set_action(
            this: &ConfigurableTaskDefinition,
            action: &ActionType,
        ) -> ConfigurableTaskDefinition;
    }

    #[wasm_bindgen(module = "hardhat/config")]
    extern "C" {
        #[wasm_bindgen(js_name = "task")]
        pub fn task(name: &str, description: &str) -> ConfigurableTaskDefinition;
    }
}

use ethabi::Contract as Abi;
use wasm_bindgen::prelude::*;

pub struct HardhatRuntimeEnvironment {
    inner: bindings::hre::HardhatRuntimeEnvironment,
}

impl HardhatRuntimeEnvironment {
    pub fn artifacts(&self) -> Artifacts {
        let artifacts = self.inner.artifacts();
        Artifacts { inner: artifacts }
    }
}

impl TryFrom<JsValue> for HardhatRuntimeEnvironment {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let inner = value.dyn_into::<bindings::hre::HardhatRuntimeEnvironment>()?;
        Ok(Self { inner })
    }
}

impl From<bindings::hre::HardhatRuntimeEnvironment> for HardhatRuntimeEnvironment {
    fn from(inner: bindings::hre::HardhatRuntimeEnvironment) -> Self {
        Self { inner }
    }
}

pub struct Artifacts {
    inner: bindings::artifacts::Artifacts,
}

impl Artifacts {
    pub async fn get_all_fully_qualified_names(&self) -> Result<Vec<String>, JsValue> {
        let result = self.inner.get_all_fully_qualified_names().await?;

        let result = js_sys::Array::from(&result);

        let result = result
            .iter()
            .map(|value| value.as_string())
            .collect::<Option<Vec<_>>>()
            .ok_or(JsValue::from_str("Failed to convert to string"))?;

        Ok(result)
    }

    pub fn read_artifact_sync(&self, name: &str) -> Result<Artifact, JsValue> {
        let artifact = Artifact::from(self.inner.read_artifact_sync(name)?);
        Ok(artifact)
    }
}

impl From<bindings::artifacts::Artifact> for Artifact {
    fn from(inner: bindings::artifacts::Artifact) -> Self {
        Self { inner }
    }
}

pub struct Artifact {
    inner: bindings::artifacts::Artifact,
}

impl Artifact {
    pub fn contract_name(&self) -> String {
        self.inner.contract_name()
    }

    pub fn abi(&self) -> Result<Abi, JsValue> {
        let abi = self.inner.abi();

        let converted: Abi = serde_wasm_bindgen::from_value(abi)?;

        Ok(converted)
    }
}
