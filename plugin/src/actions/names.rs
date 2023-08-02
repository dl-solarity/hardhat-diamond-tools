//! This module provides simple example action which gets all the
//! compiled artifacts and prints their fully qualified names.

use hardhat_bindings::HardhatRuntimeEnvironment;
use hardhat_bindings_macro::TaskParameter;
use wasm_bindgen::JsValue;

use crate::node_bindings::log;

pub const NAMES_TASK: &str = "names";
pub const NAMES_DESCRIPTION: &str = r#"
    Prints all the fully qualified names of the compiled artifacts.
"#;

#[derive(serde::Deserialize, Default, TaskParameter)]
pub struct NamesArgs {}

#[derive(Debug, thiserror::Error)]
pub enum NamesError {
    #[error("Failed to get all fully qualified names: {0:?}")]
    GetAllFullyQualifiedNames(JsValue),
}

pub async fn names_action(
    _args: NamesArgs,
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
