use crate::{
    deploy::{DeployArgs, deploy},
    new::{NewArgs, new},
};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    #[command(about = "create a new lambda function")]
    New(NewArgs),
    #[command(about = "deploy lambda function to aws")]
    Deploy(DeployArgs),
}

#[derive(Parser)]
#[command(version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::New(args) => new(args)?,
        Command::Deploy(args) => deploy(args)?,
    }

    Ok(())
}
