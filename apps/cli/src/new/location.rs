use anyhow::{Result, anyhow, bail};
use inquire::{Text, ui::RenderConfig};
use std::path::PathBuf;

const DEFAULT_LOCATION: &str = "./";

pub fn prompt(rcfg: RenderConfig) -> Result<String> {
    let location = Text::new("location")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(rcfg)
        .prompt()?;

    Ok(location)
}

pub fn get_name(path: &PathBuf) -> Result<String> {
    let file_name = path
        .file_name()
        .ok_or_else(|| anyhow!("cannot auto-detect name from `{:?}`", path.as_os_str()))?;

    let file_name_str = file_name
        .to_str()
        .ok_or_else(|| anyhow!("`{:?}` cannot contain invalid unicode", file_name))?;

    let name = file_name_str
        .trim()
        .to_lowercase()
        .replace(" ", "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
        .collect::<String>()
        .trim_matches(&['-', '_'])
        .to_string();

    if name.is_empty() {
        bail!("name must only contain alphanumeric characters, dashes, and underscores")
    }

    Ok(name)
}
