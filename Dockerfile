FROM alpine:3.20.3 as rename
WORKDIR /app
COPY target/aarch64-unknown-linux-musl/release/tira-backend tira-backend-arm64
COPY target/x86_64-unknown-linux-musl/release/tira-backend tira-backend-amd64

FROM alpine:3.20.3
ARG TARGETARCH
COPY --from=rename /app/tira-backend-$TARGETARCH /tira-backend
ENTRYPOINT [ "/tira-backend" ]
