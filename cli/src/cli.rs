use clap::{Parser, Subcommand};
use simplelog::SimpleLogger;

use crate::{actions::*, args::MergeArgs};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub(crate) struct Cli {
    /// Make the output more verbose.
    #[arg(short, long)]
    pub(crate) verbose: bool,

    #[command(subcommand)]
    pub(crate) commands: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Commands {
    Merge(MergeArgs),
}

impl Cli {
    pub(crate) fn run(self) -> eyre::Result<()> {
        self.init_logger()?;

        match self.commands {
            Commands::Merge(args) => merge(args),
        }?;

        Ok(())
    }

    fn init_logger(&self) -> eyre::Result<()> {
        let log_level = if self.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Error
        };

        SimpleLogger::init(log_level, simplelog::Config::default())?;

        Ok(())
    }
}
