# zkVM service task worker with the PCF guest program

## Quick Start

First, make sure [rustup] is installed. The
[`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to
automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following
command:

```bash
cargo run
```

### Run the proving and verification service task worker
```bash
RUST_LOG="info" RISC0_DEV_MODE=0 cargo test --release -- --nocapture
```

### Executing the project locally in development mode

During development, faster iteration upon code changes can be achieved by leveraging [dev-mode], we strongly suggest activating it during your early development phase. Furthermore, you might want to get insights into the execution statistics of your project, and this can be achieved by specifying the environment variable `RUST_LOG="info"` before running your project.

Put together, the command to run your project in development mode while getting execution statistics is:

```bash
RUST_LOG="info" RISC0_DEV_MODE=1 cargo run
```

## Directory Structure

These guest programs were used throughout the experiment:


- This is the main proving guest program highlighted in the paper: [methods/guest/src/bin/prove.rs](methods/guest/src/bin/prove.rs)
- The single step guest program: [methods/guest/src/bin/combined.rs](methods/guest/src/bin/combined.rs)
- The final composion program: [methods/guest/src/bin/compose.rs](methods/guest/src/bin/compose.rs)


## Helpful links

- [cargo-risczero](https://docs.rs/cargo-risczero)
- [risc0-build](https://docs.rs/risc0-build)
- [risc0-repo](https://www.github.com/risc0/risc0)
- [risc0-zkvm](https://docs.rs/risc0-zkvm)
- [rustup](https://rustup.rs)
- [rust-toolchain](rust-toolchain.toml)
- [zkvm-overview](https://dev.risczero.com/zkvm)


docker build . -t ghcr.io/curiousjaki/zkvm-base:latest --platform linux/amd64 -f Dockerfile_Risc0_2.3.1