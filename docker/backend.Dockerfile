FROM rust:1-bookworm AS builder

ARG APP_NAME=backend

WORKDIR /app

COPY backend/Cargo.toml backend/Cargo.lock* ./backend/
COPY backend/src ./backend/src
COPY backend/migrations ./backend/migrations

WORKDIR /app/backend

RUN cargo build --release --bin ${APP_NAME} \
    && cp target/release/${APP_NAME} /tmp/app

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /tmp/app /usr/local/bin/app
COPY --from=builder /app/backend/migrations /app/migrations

EXPOSE 8080

CMD ["/usr/local/bin/app"]
