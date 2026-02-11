# ============================================================
# Stage 1: Build the Rust binary
# ============================================================
FROM rust:1.88-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src/ ./src/
COPY migrations/ ./migrations/

# Touch main.rs so cargo knows it changed
RUN touch src/main.rs

# Build the real binary
RUN cargo build --release

# ============================================================
# Stage 2: Minimal runtime image
# ============================================================
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install runtime dependencies (SSL + CA certs for Supabase connections)
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/u-calendar-api /app/u-calendar-api

# Fly.io exposes port 8080 by default
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080

EXPOSE 8080

CMD ["/app/u-calendar-api"]