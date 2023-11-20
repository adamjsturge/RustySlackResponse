FROM rust:alpine as builder

WORKDIR /usr/src/rusty-slack-response

RUN apk add --no-cache musl-dev

COPY Cargo.toml Cargo.lock ./

COPY src/ ./src/

RUN cargo build --release --bin web

FROM alpine:latest
WORKDIR /usr/app

COPY --from=builder /usr/src/rusty-slack-response/target/release/web .

EXPOSE 3000

CMD ["./web"]
