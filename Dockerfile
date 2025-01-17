# Builder stage
FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Create a non-root user
RUN useradd -m -U -s /bin/bash appuser

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/axum-demo /app/

# Set ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose the port the app runs on
EXPOSE 8000

CMD ["./axum-demo"]
