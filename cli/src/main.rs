use clap::Parser;

mod actions;
mod args;
mod cli;

fn main() -> eyre::Result<()> {
    let cli = cli::Cli::parse();

    cli.run()
}
