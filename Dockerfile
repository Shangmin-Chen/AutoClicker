# Use the official Rust image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy only Cargo.toml and Cargo.lock for caching dependencies
COPY Cargo.toml Cargo.lock ./

# Create an empty src directory to satisfy Cargo
RUN mkdir src

# Pre-build dependencies for caching
RUN cargo build --release || true

# Copy the rest of the project files
COPY . .

# Build the full application
RUN cargo build --release

# Default command to run the application
CMD ["cargo", "run"]