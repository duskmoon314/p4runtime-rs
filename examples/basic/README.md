# basic example

This is a basic example showing how to use `p4runtime` and `p4runtime-client` to build up a simple control plane,
install 2 entries to the table and let two hosts communicate with each other.

## Prerequisites

- p4c: the P4 compiler
- bmv2: the behavioral model
  - `simple_switch_grpc` binary
- mininet: the network emulator

TODO: add more instructions

## How to run

The P4 code is compiled by `build.rs` and output to `build/`.

1. Start mininet via `sudo python basic.py`
   - You can try `h1 ping h2` and it should not work because the table is empty.
2. Run basic control plane via `cargo run --package basic`
   - This will install 2 entries to the table.
   - Try `h1 ping h2` again and it should work now.