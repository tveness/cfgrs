# cfgrs

[![Crates.io](https://img.shields.io/crates/v/cfgrs.svg?style=for-the-badge)](https://crates.io/crates/cfgrs)
[![Build status](https://img.shields.io/github/actions/workflow/status/tveness/cfgrs/rust.yml?style=for-the-badge)](https://github.com/tveness/cfgrs/actions/workflows/rust.yml)
[![License](https://img.shields.io/github/license/tveness/cfgrs?style=for-the-badge)](https://opensource.org/license/agpl-v3)
![Release](https://img.shields.io/github/v/tag/tveness/cfgrs?label=latest%20release&style=for-the-badge)


 cfgrs is a small CLI helper tool for converting between different configuration formats.
 The current formats supported are:
 * hcl
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
  hcl-rs: 0.18.0
  pico-args: 0.5.0
  serde_json: 1.0.120
  serde_yaml: 0.9.34
  toml: 0.8.15
  serde:
    features:
    - derive
    version: 1.0.204
package:
  description: CLI helper tool for converting between configuration formats
  edition: '2021'
  license: AGPL-3.0-only
  name: cfgrs
  readme: README.md
  repository: https://github.com/tveness/cfgrs
  version: 0.3.2
profile:
  release:
    codegen-units: 1
    lto: true
    opt-level: z
    panic: abort
    strip: true
 ```
 An input format may be explicitly specified, and if it isn't cfgrs will attempt to detect it.
