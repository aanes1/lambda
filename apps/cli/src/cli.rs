use crate::{
    deploy::{Deploy, deploy},
    init::{Init, init},
    new::{New, new},
};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    #[command(about = "create a new lambda function")]
    New(New),
    #[command(about = "create a new lambda function in an existing directory")]
    Init(Init),
    #[command(about = "deploy lambda function to aws")]
    Deploy(Deploy),
}

#[derive(Parser)]
#[command(version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn run() -> Result<()> {
        let cli = Cli::parse();

        match &cli.command {
            Command::New(args) => new(args)?,
            Command::Init(args) => init(args)?,
            Command::Deploy(args) => deploy(args)?,
        }

        Ok(())
    }
}
