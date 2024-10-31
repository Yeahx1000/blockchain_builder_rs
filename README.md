---

# Rust Blockchain Implementation

A simple blockchain implemented in Rust, designed to help you learn the foundations of blockchain technology. This project provides a basic blockchain with features such as proof-of-work, a CLI for interaction, and persistent storage.

This project was created as part of a personal learning journey, sharing to aid others, not intended for production use. It is a simple implementation of a blockchain, but it does not include features such as smart contracts or decentralized applications (DApps).

## Features
- **Genesis Block Creation**: Initializes the blockchain with a unique genesis block.
- **Adding New Blocks**: Append new blocks with data to the chain.
- **Proof-of-Work**: Blocks require a specified number of leading zeros in their hash, simulating a mining process.
- **Chain Validation**: Ensures the integrity of the blockchain by verifying each block’s hash and previous hash.
- **CLI Interface**: Command-line interface for users to add blocks, view the entire blockchain, validate the chain, and more.
- **Persistent Storage**: Saves the blockchain to a file so data is retained between sessions.

## Prerequisites
Basic knowledge of Rust and Blockchain technology is Preffered.
- [Rust](https://www.rust-lang.org/)
- [Blockchain Basics By IBM](https://www.ibm.com/topics/blockchain#:~:text=IBM-,What%20is%20blockchain%3F,patents%2C%20copyrights%2C%20branding)

## Installation
1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/rust-blockchain.git
   cd rust-blockchain
   ```

2. **Install dependencies**:
   Rust will manage dependencies through Cargo, Rust's package manager.

3. **Build the project**:
   ```bash
   cargo build --release
   ```

## Usage

### Running the Blockchain CLI
Use the following command to run the blockchain CLI:
```bash
cargo run -- [command] [arguments]
```

### Commands
#### 1. Initialize the Blockchain
Initialize a new blockchain with a genesis block:
```bash
cargo run -- init
```

#### 2. Add a New Block
Add a new block to the blockchain by providing data for the block:
```bash
cargo run -- add "Block data goes here"
```
Example:
```bash
cargo run -- add "Transferred 5 BTC to Alice"
```

#### 3. View the Blockchain
Display all blocks in the blockchain, showing their index, timestamp, data, and hash:
```bash
cargo run -- view
```

#### 4. Validate the Blockchain
Check the integrity of the blockchain by verifying each block’s hash and previous hash:
```bash
cargo run -- validate
```
Output will indicate whether the blockchain is valid.

#### 5. Adjust Mining Difficulty
Set the mining difficulty, which requires a specified number of leading zeros in each block’s hash:
```bash
cargo run -- set-difficulty [number]
```
Example:
```bash
cargo run -- set-difficulty 4
```

### Storage
The blockchain is saved to `blockchain.json` in the project's root directory, preserving the chain’s data between sessions. On restart, the blockchain loads from this file if it exists.

## Example Workflow
1. **Initialize the blockchain**:
   ```bash
   cargo run -- init
   ```
2. **Add blocks**:
   ```bash
   cargo run -- add "Alice pays Bob 10 BTC"
   cargo run -- add "Bob pays Charlie 5 BTC"
   ```
3. **View blockchain**:
   ```bash
   cargo run -- view
   ```
4. **Validate the blockchain**:
   ```bash
   cargo run -- validate
   ```


## Notes
This README aims to outline how to install, run, and use the Rust blockchain project, and guides users through typical operations like adding blocks, viewing, and validating the chain. Cheers!

---
