# Tic-Tac-Toe on Solana

This project implements a Tic-Tac-Toe game on the Solana blockchain using the Anchor framework.

## Project Structure

- **Rust Program:** The core logic of the Tic-Tac-Toe game is written in Rust.
- **Anchor:** The program is built using Anchor, which simplifies Solana smart contract development.
- **TypeScript Tests:** Tests are written in TypeScript to interact with the deployed program.

## Prerequisites

- **Solana CLI:** Install the Solana CLI tools from [here](https://docs.solana.com/cli/install-solana-cli-tools).
- **Anchor:** Install Anchor via Cargo: `cargo install --git https://github.com/coral-xyz/anchor --tag v0.30.1 anchor-cli --locked`.
- **Node.js:** Ensure Node.js and Yarn are installed.

## Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/Cecile-Hirschauer/tic-tac-toe.git
   cd tic-tac-toe

2. **Build the program:**

```sh
anchor build
```
3. **Deploy the program:**

```sh
anchor deploy
```

4. Testing
Run the test suite to ensure the program works as expected:

```sh
anchor test
```
The test script uses ts-mocha to run TypeScript-based tests against the local or Devnet Solana cluster.

## Configuration
- **Cluster:** The program is configured to deploy to the Solana Devnet by default.
- **Wallet:** The wallet path is specified in the Anchor.toml file.

## Program ID
The program ID for the Devnet deployment is 2VoQBkLfKDU3S7iqfo1Ub7NqBinC3fFBL8CDLqcvXz2J.

## License
This project is open-sourced under the MIT license.