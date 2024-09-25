# Phase 1 - Build the rust application
FROM rust:1.81 AS builder

WORKDIR /usr/src/app

# Install build dependencies including OpenSSL
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release

# Phase 2 - Build the final image

# Use ubuntu 22.04 which includes OpenSSL 3.x by default
FROM ubuntu:22.04

# Install necessary libraries including OpenSSL 3.x
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app

# Copy the built application from the builder image
COPY --from=builder /usr/src/app/target/release/bitcoin-puzzle-solver ./solver

# Run the application
CMD ["./solver"]