# Use an official Rust runtime as a parent image
FROM rust:1.76 as builder

# Set the working directory in the container to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Build the application
RUN sh build.sh

# Start a new stage from Debian
FROM debian:latest

# Set the working directory in the container to /app
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/output/* /app/

# Install necessary packages including PostgreSQL Client library
RUN apt-get update && apt-get install -y \
    openssl \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Make port 80 available to the world outside this container
EXPOSE 80

# Run the binary program
CMD ["sh", "-c", "/app/youtube_fetch.sh || tail -f /dev/null"]
