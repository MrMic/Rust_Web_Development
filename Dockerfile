FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update
RUN apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnux32

WORKDIR /app

COPY . .

# For musl build on M1 MAC , these ENV variables have to be set
# ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
# ENV CC='gcc'
# ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc

RUN cargo build --target x86_64-unknown-linux-musl --release

# ______________________________________________________________________

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-web-dev ./ 
COPY --from=builder /app/.env ./ 

CMD [ "/app/rust-web-dev" ]
