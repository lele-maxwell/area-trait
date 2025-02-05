FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release


FROM debian:latest

RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /app/target/release/calculator /app/

RUN chmod +x /app/calculator

CMD ["/app/payment-methods"]
