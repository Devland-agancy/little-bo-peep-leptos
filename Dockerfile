FROM rust:1.71-buster AS builder

RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  libpq-dev libpcre3-dev && \
  rm -rf /var/lib/apt/lists/*

COPY rust-toolchain.toml .
RUN rustup toolchain install nightly-2023-10-23

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin
RUN cargo binstall cargo-leptos -y
RUN rustup target add wasm32-unknown-unknown
RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN cargo leptos build --release -vv
FROM rustlang/rust:nightly-bullseye as runner
COPY --from=builder /app/target/release/little-bo-peep /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="little-bo-peep"
ENV LEPTOS_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000
# Run the server
CMD ["/app/little-bo-peep"]