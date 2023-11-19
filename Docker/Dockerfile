# Dockerfile
FROM rust:latest
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release --bin web
CMD ["./target/release/web"]
