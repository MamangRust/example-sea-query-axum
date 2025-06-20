FROM rust:1.87 as builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev 

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY ./example_sea_query ./example_sea_query
RUN cargo build --release


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && apt-get clean

WORKDIR /app

COPY --from=builder /app/target/release/example_sea_query .

COPY .env .env

EXPOSE 8000

ENV RUST_LOG=info


CMD ["./example_sea_query"]
