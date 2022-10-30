FROM rust

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release

ENV PORT=3030

EXPOSE 3030

CMD cargo run
