FROM rust:1.58-bullseye as builder
WORKDIR /usr/src/tira
COPY . .
RUN cargo install --path .

FROM debian:bullseye-20220125
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/tira /usr/local/bin/tira
CMD ["tira"]