# ============================================================
# Stage 1: Build the Rust binary
# ============================================================
FROM rust:1.83-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests first for better layer caching
COPY backend/Cargo.toml backend/Cargo.lock* ./backend/

# Create a dummy main.rs to cache dependencies
RUN mkdir -p backend/src && \
    echo "fn main() {}" > backend/src/main.rs && \
    cd backend && cargo build --release && \
    rm -rf src

# Copy actual source code
COPY backend/ ./backend/

# Touch main.rs so cargo knows it changed
RUN touch backend/src/main.rs

# Build the real binary
RUN cd backend && cargo build --release

# ============================================================
# Stage 2: Minimal runtime image
# ============================================================
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install runtime dependencies (SSL + CA certs for DB connections)
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/backend/target/release/u-calendar-api /app/u-calendar-api

# Copy migrations (embedded via include_str! but keep them for reference)
COPY backend/migrations /app/migrations

# Copy frontend files
COPY frontend /app/frontend

# Fly.io exposes port 8080 by default
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080
ENV FRONTEND_DIR=/app/frontend

EXPOSE 8080

CMD ["/app/u-calendar-api"]