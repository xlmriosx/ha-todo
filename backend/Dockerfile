# builder
FROM rust:1.87.0 AS builder
WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# runtime
FROM alpine:3.22.0 AS release
WORKDIR /usr/local/bin
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend .
COPY docker-entrypoint.sh .
RUN chmod +x docker-entrypoint.sh backend
ENV DATABASE_URL=postgres://postgres:postgres@db:5432/postgres
ENV SERVER_HOST=127.0.0.1
ENV SERVER_PORT=8080
ENV RUST_LOG=backend=debug,tower_http=debug
EXPOSE 8080
ENTRYPOINT ["sh", "./docker-entrypoint.sh"]