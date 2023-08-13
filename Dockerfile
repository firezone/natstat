FROM rust:slim

COPY . .
WORKDIR $
RUN cargo build --release

ENTRYPOINT ["./target/release/natstat"]
