use anyhow::{Result, bail};
use inquire::ui::RenderConfig;
use std::path::PathBuf;

pub fn get_render_config() -> RenderConfig<'static> {
    RenderConfig::empty()
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
