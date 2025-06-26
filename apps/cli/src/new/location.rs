use anyhow::{Result, anyhow};
use inquire::{Text, ui::RenderConfig};
use std::path::PathBuf;

const DEFAULT_LOCATION: &str = "./";

pub fn prompt(rcfg: RenderConfig) -> Result<String> {
    let location = Text::new("location")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(rcfg)
        .prompt()?
        .trim()
        .to_string();

    Ok(location)
}

pub fn get_name(path: &PathBuf) -> Result<String> {
    let file_name = path
        .file_name()
        .ok_or_else(|| anyhow!("cannot auto-detect name from `{:?}`", path.as_os_str()))?;

    let name = file_name
        .to_str()
        .ok_or_else(|| anyhow!("`{:?}` cannot have invalid unicode", file_name))?
        .to_string();

    Ok(name)
}
