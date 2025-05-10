FROM rust:alpine AS builder

WORKDIR /app

COPY . .
RUN apk --no-cache add musl-dev protoc protobuf-dev
RUN cargo build --release -p flux-messages -p flux-messages-migration

FROM alpine

WORKDIR /app

COPY --from=builder /app/target/release/flux-messages /app/
COPY --from=builder /app/target/release/flux-messages-migration /app/
COPY --from=builder /app/settings/default.toml /app/
