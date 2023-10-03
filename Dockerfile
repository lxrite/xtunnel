FROM rust:1.72 as builder
WORKDIR /usr/src/xtunnel
COPY . .
RUN cargo install --path .

FROM ubuntu:22.04
COPY --from=builder /usr/local/cargo/bin/xtunnel /usr/local/bin/xtunnel
