use diamond_tools_core::{engine::Engine, filter::IncludeExcludeFilter};
use js_sys::Promise;
use wasm_bindgen::prelude::*;

mod types;

use types::{bindings::hre::HardhatRuntimeEnvironment as Hre, HardhatRuntimeEnvironment};
use wasm_bindgen_futures::future_to_promise;

/// `console.log` for rust.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// `fs` module of `nodejs` for usage in rust.
#[wasm_bindgen(module = "node:fs")]
extern "C" {
    #[wasm_bindgen(js_name = writeFileSync, catch)]
    fn write_file_sync(path: &str, data: &str) -> Result<(), JsValue>;
}

/// The main entrypoint of the plugin. This file will be [`required`]
/// in [`hardhat.config.js`] and here all tasks will be created.
// #[wasm_bindgen(start)]
// pub fn run() -> Result<(), JsValue> {
//     task("names", "Test functionality").set_action(&names_action);

//     Ok(())
// }

#[wasm_bindgen]
pub fn names_action(_: JsValue, hre: Hre, _: JsValue) -> Promise {
    let hre = HardhatRuntimeEnvironment::from(hre);

    let artifacts = hre.artifacts();

    log("All artifacts:");

    future_to_promise(async move {
        let names = artifacts.get_all_fully_qualified_names().await?;

        for name in names {
            log(&name);
        }

        Ok(JsValue::UNDEFINED)
    })
}

#[derive(serde::Deserialize, Default)]
pub struct DiamondMergeArgs {
    pub filter: Option<IncludeExcludeFilter>,
    #[serde(rename = "outDir")]
    pub out_dir: Option<String>,
    /// The contract name to use as the base contract for the diamond
    #[serde(rename = "outContractName")]
    pub out_contract_name: Option<String>,
}

#[wasm_bindgen]
pub fn merge_artifacts_action(args: JsValue, hre: Hre, _: JsValue) -> Promise {
    let hre = HardhatRuntimeEnvironment::from(hre);

    let args: DiamondMergeArgs = match serde_wasm_bindgen::from_value(args) {
        Ok(args) => args,
        Err(e) => return Promise::reject(&format!("Failed to parse arguments: {}", e).into()),
    };

    let artifacts = hre.artifacts();

    future_to_promise(async move {
        let names = artifacts.get_all_fully_qualified_names().await?;
        let artifacts = names
            .into_iter()
            .map(|name| artifacts.read_artifact_sync(&name))
            .collect::<Result<Vec<_>, _>>()?;

        let abis = artifacts
            .into_iter()
            .map(|a| a.abi())
            .collect::<Result<Vec<_>, _>>()?;

        log("Merging artifacts...");

        let mut engine = Engine::new(abis).with_filter(args.filter.unwrap_or_default());

        engine.merge();

        let merged = engine.finish();

        let abi_json = serde_json::to_string_pretty(&merged)
            .map_err(|err| JsValue::from_str(&err.to_string()))?;

        write_file_sync(
            format!(
                "{}/{}.json",
                args.out_dir.unwrap_or_else(|| "artifacts".to_string()),
                args.out_contract_name
                    .unwrap_or_else(|| "DiamondProxy".to_string()),
            )
            .as_str(),
            &abi_json,
        )?;

        Ok(JsValue::UNDEFINED)
    })
}
