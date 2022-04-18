# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/crypto_wallet_cli

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Build the project
RUN cargo build --release

# Set the command to run the application
CMD ["./target/release/crypto_wallet_cli"]
