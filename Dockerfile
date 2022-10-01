FROM rust:1.64-slim-bullseye as builder
WORKDIR /app
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
RUN cargo init
COPY Cargo.toml Cargo.lock /app/
RUN cargo build --release
COPY src/ /app/src/
RUN find src/ -type f -exec touch {} + && cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/tira-backend /tira-backend
CMD ["/tira-backend"]
