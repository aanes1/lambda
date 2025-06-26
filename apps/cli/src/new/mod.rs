use anyhow::{Result, bail};
use clap::Parser;
use inquire::ui::RenderConfig;
use std::path::PathBuf;

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
        Some(loc) => loc.trim().to_string(),
        None => location::prompt(rcfg)?,
    };

    if location.is_empty() {
        bail!("location cannot be empty");
    }

    let path = {
        let rel = PathBuf::from(location);
        std::path::absolute(rel)?
    };
    let name = location::get_name(&path)?;

    let framework = match &args.framework {
        Some(fw) => framework::from(fw)?,
        None => framework::prompt(rcfg)?,
    };

    Ok(())
}
