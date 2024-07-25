use anyhow::{bail, Result};
use serde::Serialize;

#[derive(Debug)]
pub enum ConfigType {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ParsedInput {
    Json(serde_json::Value),
    Yaml(serde_yaml::Value),
    Toml(toml::Value),
}

pub fn try_parse_all(input: &str) -> Result<ParsedInput> {
    if let Ok(parsed) = serde_json::from_str(input) {
        Ok(ParsedInput::Json(parsed))
    } else if let Ok(parsed) = serde_yaml::from_str(input) {
        Ok(ParsedInput::Yaml(parsed))
    } else if let Ok(parsed) = toml::from_str(input) {
        Ok(ParsedInput::Toml(parsed))
    } else {
        bail!(format!(
            "Failed to parse following input as valid json, yaml, or toml: {:?}",
            input
        ))
    }
}
