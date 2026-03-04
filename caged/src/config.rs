use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Agent {
    Claude,
    Gemini,
}

impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Agent::Claude => write!(f, "claude"),
            Agent::Gemini => write!(f, "gemini"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub agent: Agent,

    #[serde(default)]
    pub packages: Vec<String>,

    #[serde(default)]
    pub mise: Vec<String>,

    #[serde(default)]
    pub volumes: Vec<String>,

    #[serde(default)]
    pub docker: bool,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read configuration file: {:?}", path))?;

        serde_yaml::from_str(&content).map_err(|e| {
            if e.to_string().contains("unknown variant") {
                return anyhow!("Unsupported agent. Supported agents: claude, gemini.");
            }

            anyhow!("Failed to parse configuration: {}", e)
        })
    }

    pub fn find_config(custom_path: Option<PathBuf>) -> Result<PathBuf> {
        let path = custom_path.unwrap_or_else(|| PathBuf::from("caged.yaml"));
        if !path.exists() {
            return Err(anyhow!(
                "Configuration file not found. Create a caged.yaml or specify one with -f."
            ));
        }

        Ok(path)
    }
}
