ARG TARGETARCH

FROM rust:1.80 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.80 AS builder-amd64
WORKDIR /app
RUN apt-get update && apt-get install -y musl-tools musl-dev
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cargo-chef --locked
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM rust:1.80 AS builder-arm64
WORKDIR /app
RUN apt-get update && apt-get install -y musl-tools musl-dev gcc-aarch64-linux-gnu
RUN rustup target add aarch64-unknown-linux-musl
RUN cargo install cargo-chef --locked
COPY --from=planner /app/recipe.json recipe.json
ENV CC_aarch64_unknown_linux_musl=aarch64-linux-gnu-gcc
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
RUN cargo chef cook --release --target aarch64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target aarch64-unknown-linux-musl

FROM scratch AS runtime-amd64
COPY --from=builder-amd64 /app/target/x86_64-unknown-linux-musl/release/first-rust-app /app
ENV PORT=8080
EXPOSE ${PORT}
ENTRYPOINT ["/app"]

FROM scratch AS runtime-arm64
COPY --from=builder-arm64 /app/target/aarch64-unknown-linux-musl/release/first-rust-app /app
ENV PORT=8080
EXPOSE ${PORT}
ENTRYPOINT ["/app"]

FROM runtime-${TARGETARCH} AS runtime