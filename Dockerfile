# Version 1 - 12 min with kaniko, 6 min with docker = 90MB
#FROM rust:1.71-slim AS planner
#WORKDIR /app
#RUN cargo install cargo-chef
#COPY . .
#RUN cargo chef prepare --recipe-path recipe.json
#
#FROM rust:1.71-slim AS cacher
#WORKDIR /app
#RUN cargo install cargo-chef
#COPY --from=planner /app/recipe.json recipe.json
#RUN cargo chef cook --release --recipe-path recipe.json
#
#FROM rust:1.71-slim AS builder
#WORKDIR /app
#COPY . /app
#COPY --from=cacher /app/target target
#COPY --from=cacher /usr/local/cargo /usr/local/cargo
#RUN cargo build --release
#
#FROM debian:buster-slim AS final
#WORKDIR /
#COPY --from=builder ./app/target/release/first-rust-app ./bin/app
#ENV PORT=8080
#EXPOSE ${PORT}
#ENTRYPOINT ["/bin/app"]

# Version 2 - scratch base image, 6 min with kaniko and docker = 21MB
FROM messense/rust-musl-cross:x86_64-musl AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch AS final
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/first-rust-app /app
ENV PORT=8080
EXPOSE ${PORT}
ENTRYPOINT ["/app"]