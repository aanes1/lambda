use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::Parser;

use crate::utils::get_render_config;

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

    Ok(())
}
