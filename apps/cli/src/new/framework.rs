use anyhow::{Result, bail};
use inquire::{Select, ui::RenderConfig};
use std::{fmt::Display, str::FromStr};

pub enum Framework {
    Axum,
}

impl Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Axum => write!(f, "axum"),
        }
    }
}

impl FromStr for Framework {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "axum" => Ok(Self::Axum),
            _ => bail!("`{}` is not a valid framework", s),
        }
    }
}

impl Framework {
    pub fn all() -> Vec<Self> {
        vec![Self::Axum]
    }
}

pub fn prompt(rcfg: RenderConfig) -> Result<Framework> {
    let framework = Select::new("framework", Framework::all())
        .with_render_config(rcfg)
        .prompt()?;
    Ok(framework)
}
