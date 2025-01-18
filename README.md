| Build |
| :---: | 
 | ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/weestack/Solana-Sniper/rust.yml) |

# Solana Sniper - A Fast and Optimized Sniping Bot in Rust

## Introduction
Welcome to the **Solana Sniper** repository! This project is part of a series of articles that guide you through building a high-performance sniper bot on the Solana blockchain using **Rust**. The goal of this sniper is to consistently place you among the top snipes for any new AMM token on **Raydium**, allowing you to capture profits from newly launched tokens.

This repository provides the base code, starting from the basics of setting up the environment to advanced optimizations for the sniper bot. The final, heavily optimized version will be shared in the last article at the end of the series.

---
# Medium Series progress

- [x] [Project structure and env setup](https://medium.com/@weestack/writing-a-raydium-sniper-in-rust-f892ce90ca69)
- [x] [Listening to token creations with websockets](https://medium.com/@weestack/building-a-lightning-fast-solana-sniper-bot-ec11dc5f2bff)
- [x] Working with initialize2 transactions, to extracting data for swapping
- [ ] Creating a Solana Command cli tool to wrap sol, and close empty accounts
- [ ] Increase stability, with enhanced error handling
- [ ] Raydium Swap instruction
- [ ] Optimized Raydium swap instruction
- [ ] Faster token discovery with (will be revealed later)
- [ ] Hosting your own RPC NODE
- [ ] Using a staked endpoint for some transactions
- [ ] Guarding against Sandwich attacks
- [ ] Implementing UI for the sniper
- [ ] Fetching Token metadata

The above is the base we need for the Sniper to be perfect, once through that we will look into creating an algorithmic trader, that sells the tokens again at the time best time (according to probability)

---

## Table of Contents
- [Features](#features)
- [Requirements](#requirements)
- [Project Structure](#project-structure)
- [Installation](#installation)
- [Running the Sniper](#running-the-sniper)
- [Contributing](#contributing)
- [License](#license)

---

## Features
- **Rust-based Sniping Bot**: A sniping bot written in Rust to achieve maximum performance and speed, outperforming JavaScript-based snipers.
- **WebSocket & RPC Integration**: Communicates with Solana and Raydium using WebSocket and RPC to listen for newly created tokens.
- **Optimizations**: Includes optimizations from Solana's manual and Helius enhanced WebSockets to reduce latency.
- **Secure Environment Handling**: Secure management of sensitive data like private keys and RPC connections through a `.env` file.
- **Future Enhancements**: The bot will be enhanced to guard against MEV sandwich attacks and further improve sniping speed and reliability.

---

## Requirements
To run this sniper bot, you'll need the following:
- **Rust**: The sniping bot is written in Rust. Please ensure you have Rust installed on your machine. If not, follow the installation guide on [Rust's website](https://www.rust-lang.org/tools/install).
- **Solana CLI**: Solana's CLI tools are required to interact with the blockchain. Install them by following the instructions [here](https://docs.solana.com/cli/install-solana-cli-tools).
- **WebSocket & RPC Endpoints**: Ensure you have access to reliable RPC and WebSocket endpoints, especially if you're using paid services for better performance.

---

## Project Structure
The project follows a modular structure with multiple binaries and a shared library for reusable code.

---

## Installation

To get started with the sniper bot:

1. Clone this repository:
    ```bash
    git clone <repository-url>
    cd solana-sniper
    ```

2. Install dependencies:
    ```bash
    cargo build --release
    ```

3. Configure the `.env` file by copying the sample from the GitHub repo or creating one manually with the required environment variables.

4. Set up WebSocket or RPC endpoints. You can use free endpoints or set up your own for better reliability.

---

## Running the Sniper

Once you've set up the environment, you can run the sniper bot with:

```bash
cargo run --release --bin sniper
```

---

## Contributing

Contributions are always welcome! If you have suggestions for new features, bug fixes, or simply wish to contribute code, feel free to submit a pull request.

---

## Supporting My Work

Your support will help me continue to develop these tools and create more useful resources for the crypto or trading community.

- **Solana (SOL)**: `[99dhtBsrX9ep8CM2H8GriNoMsRgcDwPySGfX21vHZRNP]`
- **Ethereum (ETH)**: `[0x31bf3487139bed31a646ac5863e2fa115f5d9fee]`
- **Telegram Handle** [[@weestack](https://t.me/weestack)]

---
## Disclaimer
The information provided in this series of articles is for educational purposes only and should not be considered financial or investment advice. Trading, including sniping tokens, carries significant risks, and there is no guarantee of profit.

Any decisions you make based on the content shared here are solely your responsibility. I am not liable for any financial losses incurred as a result of implementing the strategies, code, or techniques discussed in these articles.

Always trade responsibly. Never risk more money than you can afford to lose. It is essential to conduct your own research and consult with a professional financial advisor if necessary before engaging in any trading activities.
