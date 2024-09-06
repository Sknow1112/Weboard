FROM rust:1.70 as builder
WORKDIR /usr/src/weboard
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/weboard/target/release/weboard /usr/local/bin/weboard
COPY --from=builder /usr/src/weboard/static /usr/local/bin/static
WORKDIR /usr/local/bin
EXPOSE 7860
CMD ["weboard"]
