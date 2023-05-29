# Build Stage
FROM rust:1.63.0 AS builder

# Set the working directory
WORKDIR /app

# Copy the cargo manifests
COPY Cargo.toml Cargo.lock ./


# Copy the source code
COPY ./src ./src

# Build the application
RUN cargo build --release
# Final Stage

FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage

COPY --from=builder /app .

# Expose the necessary port
EXPOSE 8080

# Run the binary
CMD ["./target/release/rapier"]


# mount volume from cli 
# docker run -it -v /Users/username/Projects/rapier:/app -p 8080:8080 rapier
