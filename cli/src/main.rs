use clap::Parser;

mod actions;
mod args;
mod cli;
mod hardhat;

fn main() -> eyre::Result<()> {
    let cli = cli::Cli::parse();

    cli.run()
}
