# Morpheus code generator

This project is used to generate communication frames building and parsing code.

The frames to be used are defined in a YAML input file. The supported output languages are:
- C
- Rust

# Building

The code can be built from Nix using 
```bash
nix build
```

or from a Rust development environment using
```bash
cargo build -r
```

# Development environment

A nix development environment can be accessed from:
```bash
nix develop
```

# Code tests

The code is tested via unit tests and integration tests.
- unit tests are in the main project and can be run via:
```bash
cargo test
```

- The integration tests are in the `integration_tests` sub-folder and can be run via
```bash
cargo test -p integration-tests
```
