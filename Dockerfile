FROM rust:1.28 AS build

RUN mkdir -p /code/src
WORKDIR /code

RUN cargo install diesel_cli --no-default-features --features postgres

ADD Cargo.* .
RUN echo "fn main() {}" > /code/src/main.rs && cargo fetch

ADD src src

RUN cargo build --release

FROM debian:stretch-slim

COPY --from=build /code/target/release/todo-rs /code/gen-rs
