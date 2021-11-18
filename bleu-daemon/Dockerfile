FROM rust as builder

RUN apt update && apt install libclang-dev -y

RUN rustup component add rustfmt

WORKDIR bleu-daemon

COPY . .

RUN cargo build -j4 --release --bin bleu-daemon

FROM rust:1.56-slim as runtime

WORKDIR bleu-daemon

COPY --from=builder /bleu-daemon/config.docker.toml ./config.toml

COPY --from=builder /bleu-daemon/schema ./schema

COPY --from=builder /bleu-daemon/abi ./abi

COPY --from=builder /bleu-daemon/task ./task

COPY --from=builder /bleu-daemon/target/release/bleu-daemon .

ENTRYPOINT ["./bleu-daemon", "--config-dir", "."]

EXPOSE 9999