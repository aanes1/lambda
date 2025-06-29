use crate::utils;
use clap::Parser;
use std::path::PathBuf;

mod framework;
mod location;

#[derive(Parser)]
pub struct NewArgs {
    location: Option<String>,
    #[arg(long, alias = "fw")]
    framework: Option<String>,
}

pub fn new(args: &NewArgs) -> anyhow::Result<()> {
    let rcfg = utils::get_render_config();

    let location = match &args.location {
        Some(loc) => {
            location::check_len(&loc)?;
            loc.trim().to_string()
        }
        None => location::prompt(rcfg)?, // checks len internally
    };

    let path = PathBuf::from(location);
    location::check_path(&path)?;

    let name = location::get_name(&path)?;
    location::check_name(&name)?;

    Ok(())
}
