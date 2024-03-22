FROM messense/rust-musl-cross:x86_64-musl AS builder
#FROM rust AS builder
WORKDIR /
ENV RUST_BACKTRACE=1
ARG APP_NAME
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./nx.json .
#COPY ./apps/rust/actix_app ./apps/rust/actix_app
COPY ./apps ./apps
COPY ./libs ./libs
RUN cargo build --release -p $APP_NAME --target x86_64-unknown-linux-musl
#RUN cargo build

FROM scratch AS rust
ARG APP_NAME
#COPY --from=builder /target/x86_64-unknown-linux-musl/release/$APP_NAME  /app
#COPY --from=builder /target/release/$APP_NAME  /app
ENV PORT=8080
EXPOSE ${PORT}
ENTRYPOINT ["/app"]