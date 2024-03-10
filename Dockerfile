# Start from a Rust base image
FROM rust:1.55 as builder

# Create a new empty shell project
WORKDIR /usr/src/rust-api-gateway
COPY . .

# Build the project
RUN cargo build --release

# Start a new stage
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/myapp/target/release/rust-api-gateway /usr/local/bin/rust-api-gateway

# Set the startup command to run the binary
CMD ["rust-api-gateway"]