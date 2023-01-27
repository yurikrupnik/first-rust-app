#FROM rust:1.62 AS planner
#WORKDIR /app
#COPY . .
#RUN cargo install --path .

FROM rust:1.67 AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY .. .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.67 AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.67 AS builder
WORKDIR /app
COPY .. /app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release
#ENTRYPOINT ["/app/target/release/first-rust-app"]

#FROM base AS final
FROM debian:buster-slim
#FROM alpine:latest AS final
#FROM rust:1.62 AS final
##FROM scratch AS final
WORKDIR /
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
##COPY --from=builder /usr/src/myapp/target/release/first-rust-app ./
##COPY --from=builder ./app/target/release/first-rust-app /usr/local/bin/first-rust-app
##COPY ../target/aarch64-apple-darwin/release/first-rust-app ./first-rust-app
COPY --from=builder ./app/target/release/first-rust-app ./first-rust-app
EXPOSE 8080
##ENTRYPOINT ["/usr/bin/first-rust-app"]
CMD ["/first-rust-app"]
#ENTRYPOINT ["/app"]