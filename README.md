# P4Runtime for Rust

This repo contains the generated Rust code for the P4Runtime protobuf definitions.

## Usage

### Version

`p4runtime`'s version is composed of two parts:
- The version of this crate.
  - It will be updated when the generated Rust code differs from the previous version.
- The version of the P4Runtime protobuf definitions.
  - `1.4.0-rc.5` becomes `+1.4.0.rc.5`.

`googleapis` does not provide a version for me to use, and it seems `rpc/status.proto` has not been updated for a long time. So this may not be a problem.

## Contribution

### Repo Structure

- `src/`: The generated Rust code.
- `xtask/`: Rust code for generating this crate.

### Generate Rust Code

To generate the Rust code, run the following command:

```sh
# Get git submodule
git submodule update --init

# Generate Rust code
cargo xtask
```