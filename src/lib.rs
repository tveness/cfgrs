use serde::Serialize;
use std::str::FromStr;

#[derive(Debug)]
/// Lists different configuration file formats supported
pub enum ConfigType {
    Json,
    Yaml,
    Toml,
    Hcl,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
/// Wrapper for the different crates' parsed input e.g. `serde_json::Value` gets wrapped by `Json`
pub enum ParsedInput {
    Json(serde_json::Value),
    Yaml(serde_yaml::Value),
    Toml(toml::Value),
    Hcl(hcl::Body),
}

#[derive(Debug)]
pub struct ParseConfigError {}

impl std::fmt::Display for ParseConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse as valid hcl, json, yaml, toml")
    }
}

impl std::error::Error for ParseConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl FromStr for ParsedInput {
    type Err = ParseConfigError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        // Attempt to parse the string with each of the parsers
        if let Ok(parsed) = serde_json::from_str(s) {
            Ok(ParsedInput::Json(parsed))
        } else if let Ok(parsed) = serde_yaml::from_str(s) {
            Ok(ParsedInput::Yaml(parsed))
        } else if let Ok(parsed) = toml::from_str(s) {
            Ok(ParsedInput::Toml(parsed))
        } else if let Ok(parsed) = hcl::from_str(s) {
            Ok(ParsedInput::Hcl(parsed))
        } else {
            Err(ParseConfigError {})
        }
    }
}
