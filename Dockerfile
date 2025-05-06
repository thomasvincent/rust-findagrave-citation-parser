FROM rust:1.76-slim as builder

WORKDIR /usr/src/app
COPY . .

# Build the application with optimizations
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/findagrave-parser /app/findagrave-parser

ENTRYPOINT ["/app/findagrave-parser"]
CMD ["--help"]