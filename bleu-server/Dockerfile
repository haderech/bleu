FROM rust as builder

WORKDIR bleu-server

COPY . .

RUN cargo build --release --bin bleu-server

FROM rust as runtime

WORKDIR bleu-server

COPY --from=builder /bleu-server/.env.docker ./.env

COPY --from=builder /bleu-server/target/release/bleu-server .

ENTRYPOINT ["./bleu-server"]

EXPOSE 8888