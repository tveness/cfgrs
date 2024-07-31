//! This is the library documentation for code which is used in the binary `cfgrs`, which is likely
//! what you're looking for.
//! This can be found at https://github.com/tveness/cfgrs/releases
//! or alternatively can be installed with `cargo install cfgrs`.
//!
//!
//! cfgrs is a small CLI helper tool for converting between different configuration formats.
//! The current formats supported are:
//! * hcl
//! * json
//! * toml
//! * yaml
//!
//! These formats are not completely interchangeable, and as such if they cannot
//! be converted an error is currently raised.
//!
//! cfgrs may be used in the following way to convert between formats:
//! ```bash
//! cat Cargo.toml|cfgrs -o yaml
//! ```
//! which will then display a Cargo.toml file in its yaml representation (if this is possible)

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

/// Error type for parsing input
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
