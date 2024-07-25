use anyhow::{bail, Result};
use serde::Serialize;

const HELP: &str = "\
cfgrs is a tool to quickly convert between common configuration types, where possible.
Currently supports json, yaml, toml.



USAGE:
  cfgrs [OPTIONS] [INPUT]

OPTIONS:
  -i, --input  json|yaml|toml     specifies input type (automatically detected if not specified)
  -o, --output json|yaml|toml     specifies output type (same as input if not specified)
";

#[derive(Debug)]
enum ConfigType {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum ParsedInput {
    Json(serde_json::Value),
    Yaml(serde_yaml::Value),
    Toml(toml::Value),
}

#[derive(Debug)]
struct Args {
    input_ty: Option<ConfigType>,
    output_ty: Option<ConfigType>,
    input: String,
}

fn parse_optional_cfg_type(input: Option<String>) -> Result<Option<ConfigType>, pico_args::Error> {
    if let Some(s) = input {
        match s.as_str() {
            "json" => Ok(Some(ConfigType::Json)),
            "yaml" => Ok(Some(ConfigType::Yaml)),
            "toml" => Ok(Some(ConfigType::Toml)),
            _ => Err(pico_args::Error::OptionWithoutAValue(
                "config type must be one of json|yaml|toml",
            )),
        }
    } else {
        Ok(None)
    }
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("cfgrs {}\n\n{}", env!("CARGO_PKG_VERSION"), HELP);
        std::process::exit(0);
    }

    let args = Args {
        input_ty: parse_optional_cfg_type(pargs.opt_value_from_str(["-i", "--input"])?)?,
        output_ty: parse_optional_cfg_type(pargs.opt_value_from_str(["-o", "--output"])?)?,
        input: pargs.free_from_str()?,
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

fn main() -> Result<()> {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    // Read input into appropriate format
    let parsed_value: ParsedInput = if let Some(input_ty) = args.input_ty {
        match input_ty {
            ConfigType::Json => ParsedInput::Json(serde_json::from_str(&args.input)?),
            ConfigType::Yaml => ParsedInput::Yaml(serde_yaml::from_str(&args.input)?),
            ConfigType::Toml => ParsedInput::Toml(serde_json::from_str(&args.input)?),
        }
    } else {
        // If not specified, run through all formats and see if one works
        try_parse_all(&args.input)?
    };

    // If specified, parse into output
    let output: String = if let Some(output) = args.output_ty {
        match output {
            ConfigType::Json => serde_json::to_string(&parsed_value)?,
            ConfigType::Yaml => serde_yaml::to_string(&parsed_value)?,
            ConfigType::Toml => toml::to_string(&parsed_value)?,
        }
    } else {
        // If not specified, same as input
        match parsed_value {
            ParsedInput::Json(j) => serde_json::to_string(&j)?,
            ParsedInput::Yaml(y) => serde_yaml::to_string(&y)?,
            ParsedInput::Toml(t) => toml::to_string(&t)?,
        }
    };

    print!("{}", output);

    Ok(())
}

fn try_parse_all(input: &str) -> Result<ParsedInput> {
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
