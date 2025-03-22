FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:latest AS server

WORKDIR /app

COPY --from=builder /app/target/release/server /app/server

CMD ["./server"]

FROM debian:latest AS client

WORKDIR /app

COPY --from=builder /app/target/release/client /app/client

CMD ["./client"]
