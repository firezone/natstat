FROM rust:slim

RUN COPY . .
WORKDIR $
RUN cargo build --release

ENTRYPOINT ["./target/release/natstat"]
