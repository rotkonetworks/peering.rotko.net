FROM rust:slim-bullseye AS builder
WORKDIR /app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev curl && \
    rustup target add wasm32-unknown-unknown && \
    cargo install dioxus-cli && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN dx bundle --release --platform web --package peering-rotko-net

FROM nginx:alpine
COPY --from=builder /app/target/dx/peering-rotko-net/release/web/public /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
