# Build Stage
FROM rust:1.61 AS builder

# Set the working directory
WORKDIR /app

# Copy the cargo manifests
COPY Cargo.toml Cargo.lock ./

# Build the dependencies (cache the dependencies separately)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the source code
COPY ./src ./src

# Build the application
RUN cargo build --release

# Final Stage
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/rapier .

# Expose the necessary port
EXPOSE 8080

# Run the binary
CMD ["./rapier"]
