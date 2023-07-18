# syntax = docker/dockerfile:1.4

# Rust
FROM rust:1.61.0-slim-bullseye AS rust-builder

WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/app/target \
		--mount=type=cache,target=/usr/local/cargo/registry \
		--mount=type=cache,target=/usr/local/cargo/git \
		--mount=type=cache,target=/usr/local/rustup \
		set -eux; \
		rustup toolchain install nightly --allow-downgrade; \
		rustup target add wasm32-unknown-unknown; \
		cargo install cargo-generate; \
		npm install
	 	cargo leptos build --release; \
		objcopy --compress-debug-sections target/release/little-bo-peep-leptos ./little-bo-peep-leptos

# App f
FROM debian:11.3-slim

RUN set -eux; \
		export DEBIAN_FRONTEND=noninteractive; \
	  apt update; \
		apt install --yes --no-install-recommends bind9-dnsutils iputils-ping iproute2 curl ca-certificates htop; \
		apt clean autoclean; \
		apt autoremove --yes; \
		rm -rf /var/lib/{apt,dpkg,cache,log}/; \
		echo "Installed base utils!"

WORKDIR app

COPY --from=rust-builder /app/little-bo-peep-exp ./little-bo-peep-exp
COPY --from=elm-builder /app/dist ./dist
CMD ["./little-bo-peep-exp"]
