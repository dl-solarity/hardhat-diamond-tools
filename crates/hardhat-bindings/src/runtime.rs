use wasm_bindgen::{JsCast, JsValue};

use crate::artifacts::Artifacts;

use super::bindings;

pub struct HardhatRuntimeEnvironment {
    inner: bindings::runtime::HardhatRuntimeEnvironment,
}

impl HardhatRuntimeEnvironment {
    pub fn artifacts(&self) -> Artifacts {
        let artifacts = self.inner.artifacts();

        Artifacts::from(artifacts)
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
