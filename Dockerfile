FROM rustlang/rust:nightly-bullseye as builder
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin
RUN cargo binstall cargo-leptos -y
RUN rustup target add wasm32-unknown-unknown
RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN pwd
RUN ls -a
RUN cargo leptos build --release -vv
FROM rustlang/rust:nightly-bullseye as runner
COPY --from=builder /app/target/release/little-bo-peep /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/.env /app/
WORKDIR /app

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="little-bo-peep"
ENV LEPTOS_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000
# Run the server
#RUN mv site/pkg/little-bo-peep.*.css site/pkg/little-bo-peep.css
#RUN mv site/pkg/little-bo-peep.*.js site/pkg/little-bo-peep.js
#RUN mv site/pkg/little-bo-peep.*.wasm site/pkg/little-bo-peep.wasm

CMD ["/app/little-bo-peep"]