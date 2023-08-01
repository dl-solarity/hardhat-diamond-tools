use ethabi::Contract as Abi;
use wasm_bindgen::JsValue;

use crate::bindings;

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

impl From<bindings::artifacts::Artifacts> for Artifacts {
    fn from(inner: bindings::artifacts::Artifacts) -> Self {
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

impl From<bindings::artifacts::Artifact> for Artifact {
    fn from(inner: bindings::artifacts::Artifact) -> Self {
        Self { inner }
    }
}
