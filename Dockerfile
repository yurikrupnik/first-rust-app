FROM rust:1.80 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.80 AS builder
ARG TARGETPLATFORM
ARG TARGETOS
ARG TARGETARCH
WORKDIR /app

RUN apt-get update && apt-get install -y musl-tools musl-dev

RUN case "$TARGETARCH" in \
    "amd64") rustup target add x86_64-unknown-linux-musl ;; \
    "arm64") \
        apt-get install -y gcc-aarch64-linux-gnu && \
        rustup target add aarch64-unknown-linux-musl \
        ;; \
    esac

RUN cargo install cargo-chef --locked

COPY --from=planner /app/recipe.json recipe.json
RUN case "$TARGETARCH" in \
    "amd64") cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json ;; \
    "arm64") cargo chef cook --release --target aarch64-unknown-linux-musl --recipe-path recipe.json ;; \
    esac

COPY . .

ENV CC_aarch64_unknown_linux_musl=aarch64-linux-gnu-gcc
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc

RUN case "$TARGETARCH" in \
    "amd64") cargo build --release --target x86_64-unknown-linux-musl ;; \
    "arm64") cargo build --release --target aarch64-unknown-linux-musl ;; \
    esac

FROM scratch AS runtime
ARG TARGETARCH

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/first-rust-app /app
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/first-rust-app /app

ENV PORT=8080
EXPOSE ${PORT}
ENTRYPOINT ["/app"]