# :fire: Prometheus
[![Test](https://github.com/prometheus-lua/Prometheus/actions/workflows/Test.yml/badge.svg)](https://github.com/prometheus-lua/Prometheus/actions/workflows/Test.yml)

## Description
Prometheus is a Lua obfuscator implemented in Rust.

This project was inspired by the amazing [javascript-obfuscator](https://github.com/javascript-obfuscator/javascript-obfuscator).
It currently supports obfuscating Lua51 and Roblox's LuaU, however LuaU support is not finished yet.

You can find the full documentation including a getting started guide [here](https://levno-710.gitbook.io/prometheus/).

Prometheus has an official [Discord server](https://discord.gg/U8h4d4Rf64).

## Installation
To install Prometheus, simply clone the GitHub repository and build with Cargo:

```sh
git clone https://github.com/levno-710/Prometheus.git
cd Prometheus
cargo build --release
```

## Usage
To quickly obfuscate a script:

```sh
cargo run --bin prometheus -- --preset Medium ./your_file.lua
```

After building you can run the binary directly:

```sh
target/release/prometheus --preset Medium ./your_file.lua
```

For more advanced use cases see the [documentation](https://levno-710.gitbook.io/prometheus/).

## Tests
Run the test suite with:

```sh
cargo test
```

## License
This project is licensed under the GNU Affero General Public License v3.0. For more details, please refer to [LICENSE](https://github.com/levno-710/Prometheus/blob/master/LICENSE).
