FROM ghcr.io/jrcichra/sccache-rust:sha-959b66f as builder
ARG SCCACHE_BUCKET
ARG AWS_ACCESS_KEY_ID
ARG AWS_SECRET_ACCESS_KEY
ARG SCCACHE_REGION
ARG SCCACHE_ENDPOINT
ARG RUSTC_WRAPPER
ARG SCCACHE_LOG
ARG SCCACHE_ERROR_LOG
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN printenv && cargo build --release

FROM debian:bullseye-20220328-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/tira-backend /tira-backend
CMD ["/tira-backend"]
