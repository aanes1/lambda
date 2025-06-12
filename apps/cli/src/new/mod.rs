use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct NewArgs {
    path: Option<String>,
}

pub fn new(args: &NewArgs) -> Result<()> {
    println!("creating lambda at {:?}...", args.path);
    Ok(())
}
