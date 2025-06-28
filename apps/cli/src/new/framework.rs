use anyhow::anyhow;
use inquire::{Select, ui::RenderConfig};
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(strum::EnumIter, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Framework {
    Axum,
}

pub fn prompt(rcfg: RenderConfig) -> anyhow::Result<Framework> {
    let framework = Select::new("framework", Framework::iter().collect())
        .with_render_config(rcfg)
        .prompt()?;

    Ok(framework)
}

pub fn from(fw: &str) -> anyhow::Result<Framework> {
    let normalized = fw.trim().to_lowercase();

    if normalized.is_empty() {
        anyhow::bail!("framework cannot be empty");
    }

    let framework = Framework::from_str(&normalized)
        .map_err(|_| anyhow!("`{}` is not a valid framwork", fw))?;

    Ok(framework)
}
