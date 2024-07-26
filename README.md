# cfgrs

[![Build status](https://github.com/tveness/cfgrs/workflows/CI/badge.svg)](https://github.com/tveness/cfgrs/actions/workflows/rust.yml)

 cfgrs is a small CLI helper tool for converting between different configuration formats.
 The current formats supported are:
 * json
 * toml
 * yaml

 These formats are not completely interchangeable, and as such if they cannot
 be converted an error is currently raised.

 cfgrs may be used in the following way to convert between formats:
 ```bash
 ~/cfgrs (main ✔) cat Cargo.toml|cfgrs -o yaml
dependencies:
  anyhow: 1.0.86
  pico-args: 0.5.0
  serde_json: 1.0.120
  serde_yaml: 0.9.34
  toml: 0.8.15
  serde:
    features:
    - derive
    version: 1.0.204
package:
  edition: '2021'
  name: cfgrs
  version: 0.2.0
profile:
  release:
    codegen-units: 1
    lto: true
    opt-level: z
    panic: abort
 ```
 An input format may be explicitly specified, and if it isn't cfgrs will attempt to detect it.
