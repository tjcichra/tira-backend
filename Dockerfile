FROM rust:1.58-slim-bullseye as builder
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release

FROM debian:bullseye-20210111-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/tira-backend /tira-backend
CMD ["/tira-backend"]