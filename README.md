# Crypto Wallet CLI

This is a simple command-line interface (CLI) application for managing a cryptocurrency wallet. The application allows users to check their balance, see market prices, buy tokens, and sell tokens.

## Features

- Create a wallet with an initial balance of USDT.
- Check the balance of different tokens in the wallet.
- View current market prices of supported tokens.
- Buy and sell tokens using USDT.

## Supported Tokens

- SOL
- DOT
- ETH
- BTC
- USDT

## How to Use

1. **Clone the repository:**

    ```sh
    git clone https://github.com/DmitrySadovnikov/crypto-wallet-cli.git
    cd crypto-wallet-cli
    ```

2. **Build the Docker image:**

    ```sh
    docker build -t crypto_wallet_cli .
    ```

3. **Run the application in a Docker container:**

    ```sh
    docker run -it crypto_wallet_cli
    ```

4. **Follow the on-screen instructions:**

   - Enter your address to create a wallet.
   - Choose from the menu options to check balance, see market prices, buy tokens, or sell tokens.

## Example

```sh
📍 Please enter your address:
0x1234567890abcdef
🎉 Your wallet has been created!

1️⃣ - Check your balance
2️⃣ - See the market
3️⃣ - Buy token
4️⃣ - Sell token
5️⃣ - Exit
