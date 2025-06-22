use crate::utils::get_render_config;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod location;

#[derive(Parser)]
pub struct NewArgs {
    location: Option<String>,
}

pub fn new(args: &NewArgs) -> Result<()> {
    let rcfg = get_render_config();

    let location = match &args.location {
        Some(loc) => {
            location::check_chars(loc)?;
            loc.clone()
        }
        None => location::prompt(rcfg)?, // checks chars internally
    };

    let path = PathBuf::from(location);
    location::check_path(&path)?;

    let name = location::get_name(&path)?;

    Ok(())
}
