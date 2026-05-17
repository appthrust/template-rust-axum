FROM rust:1.88-slim AS builder

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/appthrust-rust-axum /usr/local/bin/app

ENV PORT=8080
EXPOSE 8080

CMD ["app"]
