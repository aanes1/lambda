use anyhow::Result;
use clap::Parser;
use framework::Framework;
use inquire::ui::RenderConfig;
use std::str::FromStr;

mod framework;
mod location;

#[derive(Parser)]
pub struct NewArgs {
    location: Option<String>,
    #[arg(long, alias = "fw")]
    framework: Option<String>,
}

pub fn new(args: &NewArgs) -> Result<()> {
    let rcfg = RenderConfig::empty();

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
