FROM rust:1.58.1-alpine3.15 as builder
WORKDIR /usr/src/tira
COPY . .
RUN cargo install --path .

FROM alpine:3.15
COPY --from=builder /usr/local/cargo/bin/tira /tira
CMD ["/tira"]