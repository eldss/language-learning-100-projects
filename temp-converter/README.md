# Temperature Converter

Rust CLI that converts between Celsius and Fahrenheit. Pass the target scale flag and a numeric value; the program validates the input and prints the converted temperature.

## Usage
Run from the repo root (adjust the manifest path if you are elsewhere):

```bash
cargo run --manifest-path temp-converter/rust-temp/Cargo.toml -- --celsius 100
cargo run --manifest-path temp-converter/rust-temp/Cargo.toml -- --fahrenheit 32
```

Valid flags: `--celsius`/`-c` convert °C to °F, `--fahrenheit`/`-f` convert °F to °C.

### Tests

```bash
cargo test --manifest-path temp-converter/rust-temp/Cargo.toml
```
