pub mod bindings;

use ethabi::Contract as Abi;
use wasm_bindgen::prelude::*;

pub struct HardhatRuntimeEnvironment {
    inner: bindings::runtime::HardhatRuntimeEnvironment,
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
        let inner = value.dyn_into::<bindings::runtime::HardhatRuntimeEnvironment>()?;
        Ok(Self { inner })
    }
}

impl From<bindings::runtime::HardhatRuntimeEnvironment> for HardhatRuntimeEnvironment {
    fn from(inner: bindings::runtime::HardhatRuntimeEnvironment) -> Self {
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
