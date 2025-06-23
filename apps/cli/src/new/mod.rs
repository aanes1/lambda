use crate::utils;
use anyhow::Result;
use clap::Parser;

mod location;

#[derive(Parser)]
pub struct NewArgs {
    location: Option<String>,
}

pub fn new(args: &NewArgs) -> Result<()> {
    let rcfg = utils::get_render_config();

    let location = match &args.location {
        Some(loc) => location::from(loc)?,
        None => location::prompt(rcfg)?,
    };

    Ok(())
}
