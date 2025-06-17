use anyhow::Result;
use inquire::{Text, ui::RenderConfig};

const DEFAULT_LOCATION: &str = "./";

pub fn prompt(rcfg: RenderConfig) -> Result<String> {
    let location = Text::new("location")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(rcfg)
        .prompt()?;
    Ok(location)
}
