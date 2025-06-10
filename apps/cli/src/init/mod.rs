use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Init {
    path: Option<String>,
}

pub fn init(args: &Init) -> Result<()> {
    println!("init lambda at {:?}...", args.path);
    Ok(())
}
