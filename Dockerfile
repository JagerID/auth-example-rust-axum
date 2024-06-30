FROM rust:latest

WORKDIR /app

COPY . /app

COPY /home/jager/.cargo/bin/sqlx sqlx-cli
RUN sqlx migrate run

RUN cargo build --release

CMD ['./target/release/idk']