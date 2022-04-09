FROM rust:1.60-slim-bullseye as builder
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release

FROM debian:bullseye-20220328-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/tira-backend /tira-backend
CMD ["/tira-backend"]
