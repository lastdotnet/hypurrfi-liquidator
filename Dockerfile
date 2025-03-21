# Build stage
FROM --platform=linux/amd64 rust:latest as builder

# Install Foundry
RUN curl -L https://foundry.paradigm.xyz | bash
RUN /root/.foundry/bin/foundryup

WORKDIR /usr/src/app
COPY . .

# Compile contracts with Forge (using --force and correct path)
RUN /root/.foundry/bin/forge build --root crates/liquidator-contract --force

# Build the Rust binary
RUN cargo build --release

# Runtime stage
FROM --platform=linux/amd64 debian:bookworm-slim

# Install SSL libraries and CA certificates
RUN apt-get update && \
    apt-get install -y openssl ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/aave-v3-liquidator /app/

# Make the binary executable
RUN chmod +x /app/aave-v3-liquidator

# Change to CMD to allow override from ECS
CMD ["/app/aave-v3-liquidator"]