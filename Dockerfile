FROM lukemathwalker/cargo-chef:latest-rust-1.81.0 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
# Compute a lock-like file for project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build project dependencies, not application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin ztp


# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  # Clean up
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ztp ztp
COPY configuration configuration
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./ztp"]
