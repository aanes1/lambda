use std::str::FromStr;

use crate::utils;
use anyhow::Result;
use clap::Parser;
use framework::Framework;

mod framework;
mod location;

#[derive(Parser)]
pub struct NewArgs {
    location: Option<String>,
    #[arg(long, alias = "fw")]
    framework: Option<String>,
}

pub fn new(args: &NewArgs) -> Result<()> {
    let rcfg = utils::get_render_config();

    let location = match &args.location {
        Some(loc) => location::from(loc)?,
        None => location::prompt(rcfg)?,
    };

    let framework = match &args.framework {
        Some(s) => Framework::from_str(s)?,
        None => framework::prompt(rcfg)?,
    };

    Ok(())
}
