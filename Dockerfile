FROM rust:bookworm
WORKDIR /project_root

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev curl && \
    rustup target add wasm32-unknown-unknown && \
    cargo install dioxus-cli && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN cd app && dx bundle --release --platform web --package peering-rotko-net

WORKDIR /project_root/target/dx/peering-rotko-net/release/web

RUN chmod +x ./server

ENV PORT=80
ENV IP=0.0.0.0
EXPOSE 80

CMD ["./server"]
