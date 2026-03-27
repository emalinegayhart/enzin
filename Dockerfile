FROM rust:latest as builder

WORKDIR /build

COPY Cargo.toml Cargo.toml
COPY src src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/release/enzin /app/enzin

RUN mkdir -p /data

EXPOSE 7700

VOLUME ["/data"]

ENV ENZIN_PORT=7700
ENV ENZIN_DATA_DIR=/data

CMD ["/app/enzin"]
