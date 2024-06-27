# Use the official Rust image from the Docker Hub
FROM rust:latest

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY tests ./tests

# Build the Rust project
RUN cargo build --locked

# Set the startup command to run the binary
CMD ["cargo", "test"]
