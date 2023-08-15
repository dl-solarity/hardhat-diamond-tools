//! This module provides action which merges all the compiled artifacts
//! into one with defined with args filters.

use diamond_tools_core::{
    abi::abi_to_solidity, filter::IncludeExcludeFilter, hardhat::HardhatArtifact,
    merger::DiamondMerger,
};
use ethabi::Contract;
use hardhat_bindings::HardhatRuntimeEnvironment;
use hardhat_bindings_macro::TaskParameter;
use wasm_bindgen::JsValue;

use crate::node_bindings::fs::{self, MkdirOptions};

pub const MERGE_TASK: &str = "diamond:merge";
pub const MERGE_DESCRIPTION: &str = r#"
    Merges all the compiled artifacts into one with defined with args filters.
"#;

#[derive(serde::Deserialize, TaskParameter)]
pub struct DiamondMergeArgs {
    /// Names of the methods that should be included/excluded to/from the merge.
    ///
    /// The filter is defined by `include` or `exclude` args. Only one of them
    /// can be specified.
    #[serde(rename = "filteredMethods")]
    pub filtered_methods: Option<Vec<String>>,
    /// Use this flag to include the methods into the merge
    pub include: bool,
    /// Use this flag to exclude the methods from the merge
    pub exclude: bool,
    #[serde(rename = "outDir")]
    pub out_dir: Option<String>,
    /// The contract name to use as the base contract for the diamond
    #[serde(rename = "outContractName")]
    pub out_contract_name: Option<String>,
    /// Create solidity interface of not
    #[serde(rename = "createInterface")]
    pub create_interface: bool,
}

impl Default for DiamondMergeArgs {
    fn default() -> Self {
        Self {
            filtered_methods: None,
            include: false,
            exclude: false,
            out_dir: Some(DEFAULT_OUT_DIR.to_string()),
            out_contract_name: Some(DEFAULT_OUT_CONTRACT_NAME.to_string()),
            create_interface: true,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DiamondMergeError {
    #[error("Only one of `include` or `exclude` can be specified")]
    OnlyOneFilter,
    #[error("Failed to get all fully qualified names: {0:?}")]
    GetAllFullyQualifiedNames(JsValue),
    #[error("Failed to read artifacts: {0:?}")]
    ReadArtifact(JsValue),
    #[error("Failed to parse and get abis: {0:?}")]
    ParseAbi(JsValue),
    #[error("Failed to create out dir: {0:?}")]
    CreateOutDir(JsValue),
    #[error("Failed to write merged artifact: {0}")]
    WriteArtifact(#[from] WriteError),
    #[error("Failed to create solidity interface: {0}")]
    CreateInterface(#[from] diamond_tools_core::abi::Error),
}

const DEFAULT_OUT_DIR: &str = "artifacts/contracts";
const DEFAULT_OUT_CONTRACT_NAME: &str = "DiamondProxy";

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

    log::info!("Merging abis...");

    let filters = if let Some(filter_set) = args.filtered_methods {
        if args.include && args.exclude {
            return Err(DiamondMergeError::OnlyOneFilter);
        }

        if args.include {
            IncludeExcludeFilter::from_include(filter_set)
        } else {
            IncludeExcludeFilter::from_exclude(filter_set)
        }
    } else {
        IncludeExcludeFilter::default()
    };

    let merged = DiamondMerger::new(filters).merge(abis);

    let contract_name = args
        .out_contract_name
        .expect("Default contract name is set");
    let out_dir = args.out_dir.expect("Default out dir is set");
    let dir_path = format!("{}/{}.sol", out_dir, contract_name);

    fs::mkdir(
        &dir_path,
        MkdirOptions {
            recursive: true,
            ..Default::default()
        },
    )
    .await
    .map_err(DiamondMergeError::CreateOutDir)?;

    write_merged(&contract_name, &dir_path, &merged)?;

    if !args.create_interface {
        return Ok(());
    }

    let interface = abi_to_solidity(&merged, &contract_name)?;

    log::info!("Writing solidity interface...");

    fs::write_file_sync(&format!("{}/I{}.sol", dir_path, contract_name), &interface)
        .map_err(WriteError::Write)?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum WriteError {
    #[error("Failed to form result json: {0}")]
    FormJson(#[from] serde_json::Error),
    #[error("Failed to write result file: {0:?}")]
    Write(JsValue),
}

fn write_merged(
    contract_name: &str,
    out_dir: &str,
    merged_contract: &Contract,
) -> Result<(), WriteError> {
    let hardhat_artifact = HardhatArtifact {
        contract_name: contract_name.to_string(),
        abi: merged_contract.clone(),
        ..Default::default()
    };

    let abi_json = serde_json::to_string_pretty(&hardhat_artifact)?;

    log::info!("Writing merged artifact to {}", out_dir);

    fs::write_file_sync(
        &format!("{}/{}.json", out_dir, hardhat_artifact.contract_name),
        &abi_json,
    )
    .map_err(WriteError::Write)?;

    Ok(())
}
