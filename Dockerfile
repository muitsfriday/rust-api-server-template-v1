FROM rust:latest

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app
COPY . /usr/src/app

RUN cargo build

CMD ["cargo", "run"]

EXPOSE 8000