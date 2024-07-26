# cfgrs

[![Crates.io](https://img.shields.io/crates/v/cfgrs.svg)](https://crates.io/crates/cfgrs)
[![Build status](https://img.shields.io/github/actions/workflow/status/tveness/cfgrs/CI?style=for-the-badge)](https://github.com/tveness/cfgrs/actions/workflows/rust.yml)
[![License](https://img.shields.io/github/license/tveness/cfgrs?style=for-the-badge)](https://opensource.org/license/agpl-v3)
![Release](https://img.shields.io/github/v/tag/tveness/cfgrs?label=latest%20release&style=for-the-badge)


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
