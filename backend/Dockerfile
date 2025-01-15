# Use the official Rust image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/backend

# Copy the source code into the container
COPY . .

# Build the Rust application in release mode
RUN cargo build --release

# Use a smaller base image for the final stage
FROM debian:bullseye-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin/backend

# Expose the port that the backend listens on
EXPOSE 8080

# Run the backend binary
CMD ["backend"]
