# Use the official Rust image
FROM rust:latest

# Install common system dependencies
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    lldb \
    clang \
    curl \
    git && \
    apt-get clean

# Set the working directory inside the container
WORKDIR /workspace
