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

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };
    println!("{:#?}", args);
}
