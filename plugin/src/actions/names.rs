//! This module provides simple example action which gets all the
//! compiled artifacts and prints their fully qualified names.

use hardhat_bindings_macro::hardhat_action;
use wasm_bindgen::JsValue;

use crate::node_bindings::log;

#[derive(serde::Deserialize, Default)]
pub struct NamesArgs {}

#[derive(Debug, thiserror::Error)]
pub enum NamesError {
    #[error("Failed to get all fully qualified names: {0:?}")]
    GetAllFullyQualifiedNames(JsValue),
}

#[hardhat_action]
pub async fn names_action(
    args: NamesArgs,
    hre: HardhatRuntimeEnvironment,
) -> Result<(), NamesError> {
    let artifacts = hre.artifacts();

    log("All artifacts:");

    let names = artifacts
        .get_all_fully_qualified_names()
        .await
        .map_err(NamesError::GetAllFullyQualifiedNames)?;

    for name in names {
        log(&name);
    }

    Ok(())
}
