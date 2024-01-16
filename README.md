# Typogenetics Rust

For more information about this project, please see the README for [gregorybchris/typogenetics](https://github.com/gregorybchris/typogenetics).

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Building

```bash
cargo build --release
```

## Usage

```bash
# Change to directory with build output
cd target/release

# Translate a single strand into enzymes
./typogenetics translate ATAGAGAGATCACATGTACGATAC

# Apply an enzyme to a strand to produce a set of new strands
./typogenetics rewrite cop-mvl-mvr-swi-cut-rpy AATACTAAACCGA

# Simulate many generations of evolution with a starting strand
./typogenetics simulate ATAGCGAATAGGATAATG --iter 10000 --seed 42
```
