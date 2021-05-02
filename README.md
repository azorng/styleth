# styleth

An Ethereum vanity address generator.

## Installation
Compile it yourself:
1. Install [Rust via Rustup.rs](http://rustup.rs/)
2. Clone this repository: `git clone https://github.com/azorng/styleth.git`
3. Move to the project directory: `cd styleth`
4. Compile the project: `cargo build --release`. The binary can then be found at `./target/release/styleth` or `./target/release/styleth.exe` on Windows machines.

## CLI

```
styleth 0.1.1

USAGE:
    styleth [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --match <match-value>          Matches on a given pattern where X equals any char. Example:
                                       deadXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX69
        --starts-with <starts-with>    Matches on addresses that starts with given chars. Example: dead69
```

## Development

```shell
# Build
$ cargo build

# Help
$ cargo run -- --help
```

Note: `Cargo run` creates an unoptimized executable with debug info. When testing
the speed/throughput of the application, make sure to use `cargo run --release`.
