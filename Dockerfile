FROM rust:1.77.2-bookworm as builder
WORKDIR /app
# https://users.rust-lang.org/t/cargo-uses-too-much-memory-being-run-in-qemu/76531
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
RUN cargo init
COPY Cargo.toml Cargo.lock /app/
RUN cargo build --release
COPY src/ /app/src/
RUN find src/ -type f -exec touch {} + && cargo build --release

FROM debian:bookworm
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/tira-backend /tira-backend
CMD ["/tira-backend"]
