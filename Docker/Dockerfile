FROM rust:latest

WORKDIR /usr/src/rusty-slack

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

RUN cargo build --release --bin web

EXPOSE 3000

CMD ["./target/release/web"]
