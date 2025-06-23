use crate::utils;
use anyhow::{Result, bail};
use inquire::{Text, ui::RenderConfig, validator::Validation};
use std::path::PathBuf;

const DEFAULT_LOCATION: &str = "my-lambda";

pub struct Location {
    pub path: PathBuf,
    pub name: String,
}

pub fn from(loc: &str) -> Result<Location> {
    if loc.is_empty() {
        bail!("location cannot be empty");
    }

    let rel = PathBuf::from(loc);
    let abs = std::path::absolute(rel)?;

    if abs.exists() {
        bail!("`{}` already exists", loc);
    }

    let name = utils::get_name(&abs)?;
    let path = abs.with_file_name(&name);

    Ok(Location { path, name })
}

pub fn prompt(rcfg: RenderConfig) -> Result<Location> {
    let loc = Text::new("location")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(rcfg)
        .with_validator(|loc: &str| match from(loc) {
            Ok(_) => Ok(Validation::Valid),
            Err(e) => Ok(Validation::Invalid(e.into())),
        })
        .prompt()?;

    Ok(from(&loc)?)
}
