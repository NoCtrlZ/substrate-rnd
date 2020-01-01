# substrate-rnd

A new SRML-based Substrate node, ready for hacking.

# Building

Install Rust:

Build the WebAssembly binary:

```bash
./scripts/build.sh
```

Build all native code:

```bash
cargo build
```

# Run

You can start a development chain with:

```bash
./target/release/substrate-rnd --dev
```
