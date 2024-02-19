# Create a stage for building the application.

ARG RUST_VERSION=1.76.0
ARG APP_NAME=rust-web-dev
FROM rust:${RUST_VERSION}-slim-bullseye AS builder
ARG APP_NAME
WORKDIR /app

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for 
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.

RUN --mount=type=bind,source=src,target=src \
  --mount=type=bind,source=handle-errors,target=handle-errors \
  --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
  --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  --mount=type=bind,source=migrations,target=migrations \
  <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/server
EOF

###############################################################
# FROM scratch
FROM debian:bullseye-slim AS final

COPY --from=builder /bin/server ./ 
COPY ./.env  ./

CMD ["./server"]
