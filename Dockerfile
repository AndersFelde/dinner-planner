# Get started with a build env with Rust nightly
FROM rust:alpine AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen \
    sqlite sqlite-dev sqlite-static \
    openssl openssl-dev openssl-libs-static \
    pkgconfig

RUN npm install -g sass


WORKDIR /work
COPY . .

ENV leptos_version=v0.2.43

RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/${leptos_version}/cargo-leptos-installer.sh | sh

# Add the WASM target AFTER copying sources and setting up environment
RUN rustup target add wasm32-unknown-unknown

RUN cargo leptos build --release -vv

FROM rust:alpine AS runner

WORKDIR /app

COPY --from=builder /work/target/release/dinner-planner /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

CMD ["/app/dinner-planner"]
