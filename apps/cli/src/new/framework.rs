use inquire::{Select, ui::RenderConfig};
use std::{fmt::Display, str::FromStr};

#[derive(Clone)]
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
        match s.trim().to_lowercase().as_str() {
            "axum" => Ok(Self::Axum),
            _ => anyhow::bail!("`{}` is not a valid framework", s),
        }
    }
}

impl Framework {
    pub const ALL: &[Self] = &[Self::Axum];
}

pub fn prompt(rcfg: RenderConfig) -> anyhow::Result<Framework> {
    let framework = Select::new("framework", Framework::ALL.to_vec())
        .with_render_config(rcfg)
        .prompt()?;

    Ok(framework)
}
