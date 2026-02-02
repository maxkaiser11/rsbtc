# RSBTC - Bitcoin Implementation in Rust

This project is a Bitcoin implementation in Rust, created while following the book **"Building Bitcoin in Rust"**.

## Project Structure

This workspace contains multiple related projects:

- **lib** - Core Bitcoin library with blockchain, transaction, and cryptographic primitives
- **miner** - Mining client for proof-of-work block mining
- **node** - Full node implementation
- **wallet** - Bitcoin wallet implementation
- **text_tumbler** - Transaction mixing service

## About

This is a learning project following the educational book "Building Bitcoin in Rust" to understand Bitcoin's inner workings by implementing it from scratch.

## Building

```bash
# Build all projects
cargo build

# Build a specific project
cargo build -p miner
cargo build -p node
cargo build -p wallet
```

## Running

```bash
# Run the miner
cargo run --bin miner -- block.cbor 10

# Run the node
cargo run --bin node

# Run the wallet
cargo run --bin wallet
```

## License

Educational project - following "Building Bitcoin in Rust"