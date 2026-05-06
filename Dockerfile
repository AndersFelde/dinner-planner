# Get started with a build env with Rust nightly
FROM rust:latest AS builder

RUN apt-get update -y && \
    apt-get install -y bash curl npm libc6-dev binaryen \
    sqlite3 libsqlite3-dev \
    libssl-dev \
    pkg-config

RUN npm install -g sass


WORKDIR /work
COPY . .

ENV leptos_version=v0.3.5

RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/${leptos_version}/cargo-leptos-installer.sh | sh

# Add the WASM target AFTER copying sources and setting up environment
RUN rustup target add wasm32-unknown-unknown

RUN cargo leptos build --release -vv

FROM rust:latest AS runner

RUN apt-get update -y && apt-get install -y curl \
    libgl1 libglib2.0-0 libsm6 libxext6 libxrender-dev
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV PATH="/root/.local/bin/:$PATH"

WORKDIR /app

COPY --from=builder /work/target/release/dinner-planner /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/
COPY --from=builder /work/ocr /app/ocr

# Install Python dependencies with uv
RUN cd /app/ocr && uv run python -c "from paddleocr import PaddleOCR; PaddleOCR(lang='en')"

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

CMD ["/app/dinner-planner"]
