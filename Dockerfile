# -------------------------
# 1. Builder Stage
# -------------------------
FROM rust:1.90-slim-bookworm AS builder

RUN USER=root cargo new --bin todo

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release || true

COPY . .
RUN cargo build --release

# -------------------------
# 2. Release Stage
# -------------------------
FROM debian:bookworm-slim AS release

# Set timezone to Asia/Jakarta
RUN apt-get update && apt-get install -y --no-install-recommends tzdata \
  && ln -fs /usr/share/zoneinfo/Asia/Jakarta /etc/localtime \
  && dpkg-reconfigure -f noninteractive tzdata \
  && apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/todo-rs /usr/local/bin/todo-rs

CMD ["todo-rs"]
