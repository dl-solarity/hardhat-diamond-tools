//! This module provides the CLI actions.

use std::{
    ffi::OsString,
    fs::{create_dir_all, File},
    io::Write,
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
};

use diamond_tools_core::{engine::Engine, hardhat::HardhatArtifact};
use ethabi::Contract;
use eyre::Context;
use walkdir::{DirEntry, WalkDir};

use crate::args::MergeArgs;

pub(crate) fn merge(
    MergeArgs {
        abis_path,
        follow_symlinks,
        extensions,
        include,
        exclude,
        out_dir,
        contract_name,
        create_interface,
    }: MergeArgs,
) -> eyre::Result<()> {
    let abi_pathes = read_abi_pathes_from_dir(abis_path, follow_symlinks, extensions)?;
    log::info!("Found {} ABIs", abi_pathes.len());

    let abis = read_abis(abi_pathes)?;
    log::info!("Read {} ABIs", abis.len());

    let out_dir = out_dir.unwrap_or_else(|| DEFAULT_RESULT_DIR.into());

    create_dir_all(&out_dir).wrap_err("Failed to create output directory")?;

    let merged = merge_abis(abis, include, exclude, &out_dir, &contract_name)?;

    if create_interface {
        create_and_write_interface(&merged, &out_dir, &contract_name)?;
    }

    Ok(())
}

/// Reads ABIs from the given directory. If recursivly is set to `true`, then
/// ABIs will be read from all subdirectories as well.
fn read_abi_pathes_from_dir(
    path: PathBuf,
    follow_symlinks: bool,
    extensions: Vec<String>,
) -> eyre::Result<Vec<PathBuf>> {
    let extensions = extensions
        .into_iter()
        .map(OsString::from)
        .collect::<Vec<_>>();

    let walker = WalkDir::new(path)
        .follow_links(follow_symlinks)
        .into_iter()
        .filter_entry(|e| {
            !is_hidden(e) && (has_extension(e, &extensions) || is_dir(e)) && !is_dbg_file(e)
        });

    let mut result = Vec::new();

    for entry in walker {
        let entry = entry.wrap_err("Failed to read directory entry")?;

        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        result.push(path.to_owned());
    }

    Ok(result)
}

/// Checks if the given entry is hidden (starts with a dot).
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn is_dir(entry: &DirEntry) -> bool {
    entry.path().is_dir()
}

fn has_extension(entry: &DirEntry, extensions: &[OsString]) -> bool {
    entry
        .path()
        .extension()
        .map(|ext| extensions.contains(&ext.to_os_string()))
        .unwrap_or(false)
}

/// Sufix of hardhat debug files.
///
/// For example could be somthing like: `DiamondProxy.dbg.json`. They
/// are ignored.
const DBG_SUFIX: &str = ".dbg";

/// Checks if the given entry is a hardhat debug file.
fn is_dbg_file(entry: &DirEntry) -> bool {
    entry
        .path()
        .file_stem()
        .map(|s| {
            // Check that slice is not less than the `DBG_SUFIX`.
            s.len() > DBG_SUFIX.len()
            // Check that slice ends with `DBG_SUFIX`.
                && s.as_bytes()[s.len()-DBG_SUFIX.len()..] == DBG_SUFIX.as_bytes()[..]
        })
        .unwrap_or(false)
}

/// Read and parse ABIs from the given pathes.
fn read_abis(pathes: Vec<PathBuf>) -> eyre::Result<Vec<Contract>> {
    let mut abis = Vec::with_capacity(pathes.len());

    for path in pathes {
        let abi = read_abi(path)?;

        abis.push(abi);
    }

    Ok(abis)
}

/// Read and parse ABI from the given path.
fn read_abi(path: PathBuf) -> eyre::Result<Contract> {
    let file = File::open(path.clone())
        .wrap_err_with(|| format!("Failed to open ABI file: {:?}", path.clone()))?;

    let hardhat_abi: HardhatArtifact = serde_json::from_reader(file)
        .wrap_err_with(|| format!("Failed to parse ABI file: {:?}", path.clone()))?;

    Ok(hardhat_abi.abi)
}

const DEFAULT_RESULT_DIR: &str = "artifacts";

/// Merge the abis and write the result to the given path.
fn merge_abis(
    abis: Vec<Contract>,
    includes: Option<Vec<String>>,
    excludes: Option<Vec<String>>,
    out_dir: &Path,
    contract_name: &str,
) -> eyre::Result<Contract> {
    let engine = Engine::new(abis);

    let mut engine = if let Some(includes) = includes {
        engine.with_include(includes)
    } else if let Some(excludes) = excludes {
        engine.with_exclude(excludes)
    } else {
        engine
    };

    engine.merge();

    let abi = engine.finish();

    let output = out_dir.join(format!("{}.json", contract_name));

    let file = File::create(output).wrap_err("Failed to create output file")?;
    let writer = std::io::BufWriter::new(file);

    serde_json::to_writer(writer, &abi).wrap_err("Failed to write ABI file")?;

    Ok(abi)
}

/// Create the interface for the given ABI.
fn create_and_write_interface(
    abi: &Contract,
    out_dir: &Path,
    contract_name: &str,
) -> eyre::Result<()> {
    let interface = diamond_tools_core::abi::abi_to_solidity(abi, "DiamondProxy")?;

    let output = out_dir.join(format!("I{}.sol", contract_name));

    let mut file = File::create(output).wrap_err("Failed to create output file")?;

    file.write_all(interface.as_bytes())
        .wrap_err("Failed to write interface file")?;

    Ok(())
}
