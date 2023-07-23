use diamond_tools_core::engine::Engine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DiamondMerger(Engine);

#[wasm_bindgen]
impl DiamondMerger {
    /// Accepts a list of ABIs as JSON strings and returns a merged ABI as a JSON string.
    pub fn from_raw_abi(abis: Vec<JsValue>) -> Result<DiamondMerger, String> {
        let abis = abis
            .into_iter()
            .map(|abi| abi.as_string())
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| "Failed to convert JsValue to String".to_owned())?;

        let abis = abis
            .into_iter()
            .map(|abi| serde_json::from_str(&abi))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        Ok(Self(Engine::new(abis)))
    }

    pub fn merge_and_finish(mut self) -> Result<String, String> {
        self.0.merge();
        let result = self.0.finish();

        serde_json::to_string(&result).map_err(|e| e.to_string())
    }
}
