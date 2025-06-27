use anyhow::{Result, anyhow, bail};
use inquire::{Select, ui::RenderConfig};
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(strum::EnumIter, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Framework {
    Axum,
    Actix,
    Rocket,
}

pub fn prompt(rcfg: RenderConfig) -> Result<Framework> {
    let framework = Select::new("framework", Framework::iter().collect())
        .with_render_config(rcfg)
        .prompt()?;

    Ok(framework)
}

pub fn from(fw: &str) -> Result<Framework> {
    let normalized = fw.trim().to_lowercase();

    if normalized.is_empty() {
        bail!("framework cannot be empty");
    }

    let framework = Framework::from_str(&normalized)
        .map_err(|_| anyhow!("`{}` is not a valid framwork", fw))?;

    Ok(framework)
}
