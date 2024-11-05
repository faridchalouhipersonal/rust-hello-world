# Use the official Rust image as the base
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy only the necessary files to the container
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the application
RUN cargo build --release

# Expose the application port
EXPOSE 8080

# Command to run the application
CMD ["./target/release/rust-web-server"]

