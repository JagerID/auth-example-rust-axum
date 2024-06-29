FROM rust:latest

WORKDIR /app

COPY . /app

RUN cargo install sqlx-cli
RUN sqlx migrate run

RUN cargo build --release

CMD ['./target/release/idk']