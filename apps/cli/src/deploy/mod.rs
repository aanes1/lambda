use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Deploy {
    path: Option<String>,
}

pub fn deploy(args: &Deploy) -> Result<()> {
    println!("deploying lambda at {:?}...", args.path);
    Ok(())
}
