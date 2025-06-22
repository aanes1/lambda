use anyhow::{Result, bail};
use inquire::{Text, ui::RenderConfig, validator::Validation};
use std::path::PathBuf;

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
