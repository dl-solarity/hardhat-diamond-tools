//! This module provides action which merges all the compiled artifacts
//! into one with defined with args filters.

use diamond_tools_core::{engine::Engine, filter::IncludeExcludeFilter};
use ethabi::Contract;
use hardhat_bindings_macro::hardhat_action;
use wasm_bindgen::JsValue;

use crate::node_bindings::{
    fs::{self, MkdirOptions},
    log,
};

pub const MERGE_TASK: &str = "diamond:merge";
pub const MERGE_DESCRIPTION: &str = r#"
    Merges all the compiled artifacts into one with defined with args filters.
"#;

#[derive(serde::Deserialize, Default)]
pub struct DiamondMergeArgs {
    pub filter: Option<IncludeExcludeFilter>,
    #[serde(rename = "outDir")]
    pub out_dir: Option<String>,
    /// The contract name to use as the base contract for the diamond
    #[serde(rename = "outContractName")]
    pub out_contract_name: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum DiamondMergeError {
    #[error("Failed to get all fully qualified names: {0:?}")]
    GetAllFullyQualifiedNames(JsValue),
    #[error("Failed to read artifacts: {0:?}")]
    ReadArtifact(JsValue),
    #[error("Failed to parse and get abis: {0:?}")]
    ParseAbi(JsValue),
    #[error("Failed to merge artifacts: {0:?}")]
    MergeArtifacts(#[from] serde_json::Error),
    #[error("Failed to write merged artifact: {0:?}")]
    WriteArtifact(JsValue),
    #[error("Failed to create out dir: {0:?}")]
    CreateOutDir(JsValue),
}

const DEFAULT_OUT_DIR: &str = "artifacts/contracts";
const DEFAULT_OUT_CONTRACT_NAME: &str = "DiamondProxy";

#[hardhat_action]
pub async fn merge_artifacts_action(
    args: DiamondMergeArgs,
    hre: HardhatRuntimeEnvironment,
) -> Result<(), DiamondMergeError> {
    let artifacts = hre.artifacts();

    let names = artifacts
        .get_all_fully_qualified_names()
        .await
        .map_err(DiamondMergeError::GetAllFullyQualifiedNames)?;
    let artifacts = names
        .into_iter()
        .map(|name| artifacts.read_artifact_sync(&name))
        .collect::<Result<Vec<_>, _>>()
        .map_err(DiamondMergeError::ReadArtifact)?;

    let abis = artifacts
        .into_iter()
        .map(|a| a.abi())
        .collect::<Result<Vec<_>, _>>()
        .map_err(DiamondMergeError::ParseAbi)?;

    log("Merging artifacts...");

    let mut engine = Engine::new(abis).with_filter(args.filter.unwrap_or_default());

    engine.merge();

    let merged = engine.finish();

    write_merged(args.out_contract_name, args.out_dir, merged).await?;

    Ok(())
}

async fn write_merged(
    out_contract_name: Option<String>,
    out_dir: Option<String>,
    merged_contract: Contract,
) -> Result<(), DiamondMergeError> {
    let abi_json = serde_json::to_string_pretty(&merged_contract)?;

    let contract_name = out_contract_name.unwrap_or_else(|| DEFAULT_OUT_CONTRACT_NAME.to_string());
    let out_dir = out_dir.unwrap_or_else(|| DEFAULT_OUT_DIR.to_string());
    let dir_path = format!("{}/{}.sol", out_dir, contract_name);

    log(format!("Writing merged artifact to {}", dir_path).as_str());

    fs::mkdir(
        &dir_path,
        MkdirOptions {
            recursive: true,
            ..Default::default()
        },
    )
    .await
    .map_err(DiamondMergeError::CreateOutDir)?;

    fs::write_file_sync(
        format!("{}/{}.json", dir_path, contract_name).as_str(),
        &abi_json,
    )
    .map_err(DiamondMergeError::WriteArtifact)?;

    Ok(())
}
