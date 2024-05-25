FROM rust:slim-bullseye as builder

WORKDIR /metro_map_site

ENV RUST_BACKTRACE=1

RUN apt-get update \
    && apt-get install -y pkg-config libssl-dev \
    && cargo install --locked trunk \
    && rustup toolchain install nightly \
    && rustup default nightly \
    && rustup target add wasm32-unknown-unknown 

COPY . .

RUN trunk build --release

FROM nginx:alpine as target

COPY --from=builder /metro_map_site/dist/ /usr/share/nginx/html

