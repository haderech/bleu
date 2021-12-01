FROM rust as builder

RUN apt update && apt install apt-utils libclang-dev -y

RUN rustup component add rustfmt

WORKDIR bleu-daemon

COPY . .

RUN cargo build -j4 --release --bin bleu-daemon

FROM rust:1.56-slim as runtime

RUN apt update && apt install apt-utils -y

WORKDIR bleu-daemon

COPY --from=builder /bleu-daemon/target/release/bleu-daemon .

ENV RUST_LOG INFO

ENTRYPOINT ["./bleu-daemon", "--config-dir", "."]

EXPOSE 9999