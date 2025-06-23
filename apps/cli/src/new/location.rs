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

    let name = get_name(&abs)?;
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

pub fn get_name(abs: &PathBuf) -> Result<String> {
    let file_name = abs.file_name().ok_or_else(|| {
        anyhow::format_err!(
            "cannot auto-detect project name from `{:?}`",
            abs.as_os_str()
        )
    })?;

    let file_name_str = file_name
        .to_str()
        .ok_or_else(|| anyhow::format_err!("`{:?}` cannot contain invalid unicode", file_name))?;

    let name = file_name_str
        .trim()
        .to_lowercase()
        .replace(" ", "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>()
        .trim_matches(&['-', '_'])
        .to_string();

    if name.is_empty() {
        bail!("project name must only contain alphanumeric characters, dashes, and underscores")
    }

    Ok(name)
}
