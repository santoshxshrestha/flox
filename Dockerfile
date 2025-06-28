FROM rust:1.87.0 as builder

WORKDIR /usr/src/flox
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /flox
COPY --from=builder /usr/src/flox/target/release/flox ./flox
COPY --from=builder /usr/src/flox/static ./static
COPY --from=builder /usr/src/flox/templates ./templates

RUN chmod +x ./flox

ENV RUST_LOG=info
ENV ACTIX_ENV=production

EXPOSE 8080

CMD ["./flox"]

