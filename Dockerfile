FROM rust:alpine as Base

WORKDIR /usr/src/app

COPY src ./src
COPY Cargo.toml .
COPY Cargo.lock .

RUN apk add --no-cache musl-dev

RUN cargo build --release

FROM alpine:latest as Runner

WORKDIR /usr/src/app

COPY ./config ./config
COPY --from=Base /usr/src/app/target/release/static-web-server .

RUN addgroup -S appgroup && adduser -S appuser -G appgroup
RUN chown -R 1001:1001 /usr/src/app
USER 1001

CMD ["./static-web-server"]