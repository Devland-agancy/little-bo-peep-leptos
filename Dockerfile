FROM rust:1.71-buster AS builder

RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  libpq-dev libpcre3-dev && \
  rm -rf /var/lib/apt/lists/*

# The toolchain version must manually be kept in sync with rust-toolchain.toml
COPY rust-toolchain.toml .
RUN rustup toolchain install nightly-2023-09-19

RUN cargo install --locked cargo-leptos
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown
RUN mkdir -p /app
WORKDIR /app
COPY . .
ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-gnu"
RUN cargo leptos --manifest-path=./Cargo.toml build --release -vv

FROM rust:1.71-buster as runner
COPY --from=builder /app/target/server/x86_64-unknown-linux-gnu/release/little-bo-peep /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="little-bo-peep"
ENV LEPTOS_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000
CMD ["/app/little-bo-peep"]