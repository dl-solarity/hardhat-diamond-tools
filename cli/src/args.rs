use std::path::PathBuf;

use clap::Parser;

/// Arguments for the `merge` subcommand.
#[derive(Parser, Debug, Clone)]
pub(crate) struct MergeArgs {
    /// The methods of the contract that should be included.
    /// If none are specified, all methods are included.
    #[arg(short, long, group = "filter")]
    pub(crate) include: Option<Vec<String>>,

    /// The methods of the contract that should be excluded.
    /// If none are specified, no methods are excluded.
    #[arg(short, long, group = "filter")]
    pub(crate) exclude: Option<Vec<String>>,

    /// Whether to follow symlinks when reading ABIs from a directory.
    #[arg(short, long)]
    pub(crate) follow_symlinks: bool,

    /// The path to the output file.
    #[arg(short, long)]
    pub(crate) output: Option<PathBuf>,

    /// A list of possible file extensions to read ABIs from.
    #[arg(long, default_value = "json")]
    pub(crate) extensions: Vec<String>,

    /// The path to ABIs directory to use. Only ABIs with `.json`  extensions will be read.
    pub(crate) abis_path: PathBuf,
}
