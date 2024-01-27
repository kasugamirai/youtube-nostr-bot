    # use rust:latest as builder
    FROM --platform=linux/amd64 rust:latest as builder

    # set working directory
    WORKDIR /app

    # copy source code to working directory
    COPY . .

    # build release binary
    RUN cargo build --release

    # copy binary to alpine image
    FROM --platform=linux/amd64 alpine:latest

    # set working directory
    WORKDIR /app

    # copy binary from builder
    COPY --from=builder /app/target/release/youtube_bot_run .
