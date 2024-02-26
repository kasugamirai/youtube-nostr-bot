# Use an official Rust runtime as a parent image
FROM rust:1.76

# Set the working directory in the container to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Build the application
RUN cargo build --release

# Make port 80 available to the world outside this container
EXPOSE 80

# Run the binary program produced by `cargo build`
CMD ["/app/target/release/bootstrap"]