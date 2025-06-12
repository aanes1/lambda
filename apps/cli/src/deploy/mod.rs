use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct DeployArgs {
    path: Option<String>,
}

pub fn deploy(args: &DeployArgs) -> Result<()> {
    println!("deploying lambda at {:?}...", args.path);
    Ok(())
}
