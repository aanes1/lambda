use anyhow::{Result, bail};
use inquire::{Text, ui::RenderConfig, validator::Validation};
use std::path::{PathBuf, absolute};

const DEFAULT_LOCATION: &str = "./";

pub fn check_chars(s: &str) -> Result<()> {
    if s.is_empty() {
        bail!("location cannot be empty");
    }
    // https://stackoverflow.com/a/31976060
    if s.chars().any(|c| ":;<>\"|?*".contains(c)) {
        bail!("`{}` cannot contain special characters", s);
    }
    Ok(())
}

pub fn prompt(rcfg: RenderConfig) -> Result<String> {
    let location = Text::new("location")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(rcfg)
        .with_validator(|s: &str| match check_chars(s) {
            Ok(()) => Ok(Validation::Valid),
            Err(e) => Ok(Validation::Invalid(e.into())),
        })
        .prompt()?;
    Ok(location)
}

pub fn check_path(path: &PathBuf) -> Result<()> {
    if path.is_file() {
        bail!("`{}` is a file that already exists", path.display());
    }
    if path.is_dir() {
        bail!(
            "`{}` is a directory that already exists, use `lambda init` to initialize the directory",
            path.display()
        );
    }
    if path.exists() {
        bail!("`{}` already exists", path.display())
    }
    Ok(())
}

pub fn get_name(path: &PathBuf) -> Result<String> {
    let absolute_path = absolute(path)?;

    let file_name = absolute_path.file_name().ok_or_else(|| {
        anyhow::format_err!(
            "failed to auto-detect project name from `{:?}`",
            absolute_path.as_os_str()
        )
    })?;
    let file_name_str = file_name.to_str().ok_or_else(|| {
        anyhow::format_err!("`{:?}` cannot contain non-unicode characters", file_name)
    })?;

    let name = file_name_str.trim().to_lowercase().replace(" ", "-");
    Ok(name)
}

pub fn check_name(name: &str) -> Result<()> {
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        bail!(
            "`{}` must only contain alphanumeric characters, dashes, and underscores",
            name
        );
    }
    if name.starts_with(&['-', '_']) || name.ends_with(&['-', '_']) {
        bail!("`{}` cannot start or end with dashes or underscores", name)
    }
    Ok(())
}
