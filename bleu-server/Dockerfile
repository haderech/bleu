FROM rust as builder

RUN apt update && apt install apt-utils build-essential -y

WORKDIR bleu-server

COPY . .

RUN cargo build --release --bin bleu-server

FROM rust:1.56-slim as runtime

RUN apt update && apt install apt-utils libpq-dev -y

WORKDIR bleu-server

COPY --from=builder /bleu-server/target/release/bleu-server .

COPY --from=builder /bleu-server/swagger-ui ./swagger-ui

ENV RUST_LOG INFO

ENTRYPOINT ["./bleu-server"]

EXPOSE 8888