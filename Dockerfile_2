FROM rust:latest

# COPY ./ ./
COPY . .

# ENV CARGO_HTTP_MULTIPLEXING=false

RUN cargo build --release

CMD [ "./target/release/rust-web-dev" ]
