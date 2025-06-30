# builder
FROM rust:1.87.0 AS builder
WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# runtime
FROM alpine:3.22.0 AS release
WORKDIR /usr/local/bin
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend .
COPY docker-entrypoint.sh .
RUN chmod +x docker-entrypoint.sh backend
EXPOSE 8080
ENTRYPOINT ["./docker-entrypoint.sh"]