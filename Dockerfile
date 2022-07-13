#FROM rust:1.62 as builder
#WORKDIR /usr/src/myapp
#COPY . .
#RUN cargo build --release
#RUN USER=root cargo new --bin holodeck
# copy over your manifests
#COPY ./Cargo.lock ./Cargo.lock
#COPY ./Cargo.toml ./Cargo.toml
# this build step will cache your dependencies
#RUN cargo install --path .
#RUN rm src/*.rs
#/RUN cargo install --path .
#RUN #cargo install --path .

#FROM debian:buster-slim
FROM alpine:latest
WORKDIR /
RUN #apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
#COPY --from=builder /usr/src/myapp/target/release/first-rust-app ./
COPY  ./target/release/first-rust-app ./app
EXPOSE 8080
#ENTRYPOINT ["/usr/bin/first-rust-app"]
#ENTRYPOINT ["/app"]