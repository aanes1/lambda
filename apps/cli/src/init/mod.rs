use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct InitArgs {
    path: Option<String>,
}

pub fn init(args: &InitArgs) -> Result<()> {
    println!("init lambda at {:?}...", args.path);
    Ok(())
}
