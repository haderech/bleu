FROM rust as builder

RUN apt update && apt install apt-utils build-essential -y

WORKDIR cv-server

COPY . .

RUN cargo build --release --bin cv-server

FROM rust:1.56-slim as runtime

RUN apt update && apt install apt-utils libpq-dev -y

WORKDIR cv-server

COPY --from=builder /cv-server/target/release/cv-server .

COPY --from=builder /cv-server/swagger-ui ./swagger-ui

ENV RUST_LOG INFO

ENTRYPOINT ["./cv-server"]

EXPOSE 8888
