use anyhow::anyhow;
use inquire::{Text, ui::RenderConfig, validator::Validation};
use std::path::PathBuf;

const MAX_LOCATION_LEN: usize = 255;
const DEFAULT_LOCATION: &str = "./";

pub fn check_len(loc: &str) -> anyhow::Result<()> {
    let trimmed = loc.trim();

    if trimmed.is_empty() {
        anyhow::bail!("location cannot be empty");
    }

    if trimmed.len() > MAX_LOCATION_LEN {
        anyhow::bail!(
            "location cannot be more than {} characters",
            MAX_LOCATION_LEN
        );
    }

    Ok(())
}

pub fn prompt(rcfg: RenderConfig) -> anyhow::Result<String> {
    let location = Text::new("location")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(rcfg)
        .with_validator(|loc: &str| match check_len(loc) {
            Ok(()) => Ok(Validation::Valid),
            Err(e) => Ok(Validation::Invalid(e.into())),
        })
        .prompt()?
        .trim()
        .to_string();

    Ok(location)
}

pub fn check_path(path: &PathBuf) -> anyhow::Result<()> {
    if path.is_file() {
        anyhow::bail!("`{}` is a file that already exists", path.display());
    }

    if std::env::join_paths(path).is_err() {
        anyhow::bail!("`{}` contains invalid path characters", path.display());
    }

    Ok(())
}

pub fn get_name(path: &PathBuf) -> anyhow::Result<String> {
    let abs = std::path::absolute(path)?;

    let file_name = abs
        .file_name()
        .ok_or_else(|| anyhow!("cannot auto-detect name from `{}`", path.display()))?;

    let name = file_name
        .to_str()
        .ok_or_else(|| anyhow!("`{:?}` contains invalid unicode", file_name))?
        .to_string();

    Ok(name)
}

pub fn check_name(name: &str) -> anyhow::Result<()> {
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!(
            "`{}` must only contain alphanumeric characters, dashes, and underscores",
            name
        );
    }

    if name.starts_with('-') || name.ends_with('-') {
        anyhow::bail!("`{}` cannot start or end with dash", name);
    }

    Ok(())
}
