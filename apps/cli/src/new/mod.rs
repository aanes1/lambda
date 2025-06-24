use anyhow::{Result, bail};
use clap::Parser;
use inquire::ui::RenderConfig;
use std::path::PathBuf;

mod location;

#[derive(Parser)]
pub struct NewArgs {
    location: Option<String>,
}

pub fn new(args: &NewArgs) -> Result<()> {
    let rcfg = RenderConfig::empty();

    let location = match &args.location {
        Some(loc) => loc.clone(),
        None => location::prompt(rcfg)?,
    };

    if location.is_empty() {
        bail!("location cannot be empty");
    }

    let mut path = {
        let rel = PathBuf::from(location);
        std::path::absolute(rel)?
    };

    let name = location::get_name(&path)?;
    path.set_file_name(&name);

    Ok(())
}
