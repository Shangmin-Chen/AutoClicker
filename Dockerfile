# Use the latest Rust image
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy dependency files first to leverage Docker caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main file for dependency resolution
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release || true

# Copy the actual source files
COPY . .

# Build the application
RUN cargo build --release

# Set the default command
CMD ["./target/release/app"]
