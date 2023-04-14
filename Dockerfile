FROM rust:latest

WORKDIR /app

COPY . .
COPY training-data ./training-data

RUN cargo build --release

CMD ["cargo", "run"]
