use clap::Parser;

mod actions;
mod cli;
mod hardhat;

fn main() -> eyre::Result<()> {
    let cli = cli::Cli::parse();

    cli.run()
}
