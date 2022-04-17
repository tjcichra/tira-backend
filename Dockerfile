FROM ghcr.io/jrcichra/sccache-rust:sha-959b66f as builder
WORKDIR /usr/src/app
ENV SCCACHE_BUCKET=${SCCACHE_BUCKET} \
    AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID} \
    AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY} \
    SCCACHE_REGION=${SCCACHE_REGION} \
    SCCACHE_ENDPOINT=${SCCACHE_ENDPOINT} \
    RUSTC_WRAPPER=${RUSTC_WRAPPER} \
    SCCACHE_LOG=${SCCACHE_LOG} \
    SCCACHE_ERROR_LOG=${SCCACHE_ERROR_LOG}
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release

FROM debian:bullseye-20220328-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/tira-backend /tira-backend
CMD ["/tira-backend"]
