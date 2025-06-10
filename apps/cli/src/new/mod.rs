use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct New {
    path: Option<String>,
}

pub fn new(args: &New) -> Result<()> {
    println!("creating lambda at {:?}...", args.path);
    Ok(())
}
