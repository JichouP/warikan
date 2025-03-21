FROM lukemathwalker/cargo-chef:latest-rust-1.85.1-slim-bookworm AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config=1.8.1-1 libssl-dev=3.0.15-1~deb12u1 --no-install-recommends && rm -rf /var/lib/apt/lists/*

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM chef AS builder
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:stable-slim AS build-env

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
COPY --from=builder /app/target/release/main /app
COPY --from=build-env /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1
CMD ["/app/main"]
