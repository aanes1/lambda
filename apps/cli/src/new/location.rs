use std::path::PathBuf;

use anyhow::{Result, bail};
use inquire::{Text, ui::RenderConfig};

const DEFAULT_LOCATION: &str = "./";

pub fn prompt(rcfg: RenderConfig) -> Result<String> {
    let location = Text::new("location")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(rcfg)
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
