FROM rust:bookworm AS builder
WORKDIR /app
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev curl && \
    rustup target add wasm32-unknown-unknown && \
    cargo install dioxus-cli && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
COPY . .
RUN dx bundle --release --platform web --package peering-rotko-net

FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/dx/peering-rotko-net/release/web/ /usr/local/app
RUN chmod +x /usr/local/app/server
ENV PORT=80
ENV IP=0.0.0.0
EXPOSE 80
WORKDIR /usr/local/app
ENTRYPOINT ["/usr/local/app/server"]
