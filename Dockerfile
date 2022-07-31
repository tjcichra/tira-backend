FROM ghcr.io/jrcichra/sccache-rust:sha-240e206 as builder
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
RUN (touch /tmp/sccache_log.txt && tail -f /tmp/sccache_log.txt &) && printenv && cargo build --release -j8

FROM gcr.io/distroless/base-debian11
COPY --from=builder /usr/src/app/target/release/tira-backend /tira-backend
CMD ["/tira-backend"]
