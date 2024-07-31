use cfgrs::{try_parse_all, ConfigType, ParsedInput};
use std::io::Read;

use anyhow::{Context, Result};

const HELP: &str = "\
cfgrs is a tool to quickly convert between common configuration types, where possible.
Currently supports hcl, json, toml, yaml.

USAGE:
  cfgrs [OPTIONS] [INPUT]

OPTIONS:
  -i, --input  hcl|json|toml|yaml     specifies input type (automatically detected if not specified)
  -o, --output hcl|json|toml|yaml     specifies output type (same as input if not specified)
";

#[derive(Debug)]
struct Args {
    input_ty: Option<ConfigType>,
    output_ty: Option<ConfigType>,
    input: String,
}

fn parse_optional_cfg_type(input: Option<String>) -> Result<Option<ConfigType>, pico_args::Error> {
    if let Some(s) = input {
        match s.as_str() {
            "hcl" => Ok(Some(ConfigType::Hcl)),
            "json" => Ok(Some(ConfigType::Json)),
            "toml" => Ok(Some(ConfigType::Toml)),
            "yaml" => Ok(Some(ConfigType::Yaml)),
            _ => Err(pico_args::Error::OptionWithoutAValue(
                "config type must be one of hcl|json|toml|yaml",
            )),
        }
    } else {
        Ok(None)
    }
}

/// Parses all of the arguments
fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Exit early for help dialogue
    if pargs.contains(["-h", "--help"]) {
        print!("cfgrs {}\n\n{}", env!("CARGO_PKG_VERSION"), HELP);
        std::process::exit(0);
    }

    let input_ty = parse_optional_cfg_type(pargs.opt_value_from_str(["-i", "--input"])?)?;
    let output_ty = parse_optional_cfg_type(pargs.opt_value_from_str(["-o", "--output"])?)?;

    // Read input from args, or from stdin if there aren't any there
    let input = if let Ok(free_args) = pargs.free_from_str() {
        free_args
    } else {
        let mut buffer = vec![];
        std::io::stdin().read_to_end(&mut buffer).map_err(|e| {
            pico_args::Error::ArgumentParsingFailed {
                cause: format!("{e}"),
            }
        })?;
        std::str::from_utf8(&buffer)
            .map_err(|e| pico_args::Error::ArgumentParsingFailed {
                cause: format!("{e}"),
            })?
            .to_string()
    };

    let args = Args {
        input_ty,
        output_ty,
        input,
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
            ConfigType::Json => ParsedInput::Json(
                serde_json::from_str(&args.input).context("parsing input to json")?,
            ),
            ConfigType::Yaml => ParsedInput::Yaml(
                serde_yaml::from_str(&args.input).context("parsing input to yaml")?,
            ),
            ConfigType::Toml => ParsedInput::Toml(
                serde_json::from_str(&args.input).context("parsing input to toml")?,
            ),
            ConfigType::Hcl => {
                ParsedInput::Hcl(hcl::from_str(&args.input).context("parsing input to hcl")?)
            }
        }
    } else {
        // If not specified, run through all formats and see if one works
        try_parse_all(&args.input)?
    };

    // If specified, parse into output
    let output: String = match (args.output_ty, &parsed_value) {
        (Some(ConfigType::Json), _) | (None, ParsedInput::Json(_)) => {
            serde_json::to_string(&parsed_value).context("converting to json")?
        }

        (Some(ConfigType::Yaml), _) | (None, ParsedInput::Yaml(_)) => {
            serde_yaml::to_string(&parsed_value).context("converting to yaml")?
        }
        (Some(ConfigType::Toml), _) | (None, ParsedInput::Toml(_)) => {
            toml::to_string(&parsed_value).context("converting to toml")?
        }
        (Some(ConfigType::Hcl), _) | (None, ParsedInput::Hcl(_)) => {
            hcl::to_string(&parsed_value).context("converting to hcl")?
        }
    };

    print!("{}", output);

    Ok(())
}
