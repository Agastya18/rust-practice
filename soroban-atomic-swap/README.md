# Soroban Atomic Swap Smart Contract Project

This repository contains a proof of concept (PoC) for an atomic-swap Soroban smart contract project for a client. The atomic swap example swaps two tokens between two authorized parties atomically while following the limits they set.

## Prerequisites

Before running this project, ensure you have the setup ready:

- [Soroban Setup](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)

## Getting Started

1. **Clone the repository**:
   ```bash
   git clone https://github.com/agastya18/soroban-atomic-swap
   ```

2. **Initialize the project**:
   Ensure all dependencies are properly set up by running:
   ```bash
   cargo build
   ```

3. **Add a new Soroban contract**:
   You can add new contracts by creating new subdirectories under `contracts/` and adding the necessary files (`Cargo.toml`, `lib.rs`, etc.).

4. **Running Tests**:
   Each contract comes with its own test suite. Navigate to the contract's directory and run:
   ```bash
   cargo test
   ```

## Project Structure

- `contracts/`: Contains the smart contracts.
  - `swap/`: Example contract for atomic swap.
    - `src/`: Source code for the contract.
      - `lib.rs`: Main contract implementation.
      - `test.rs`: Test cases for the contract.

## Example

The `swap` contract demonstrates an atomic swap between two tokens. The test file `test.rs` includes a comprehensive test case to validate the swap functionality.
