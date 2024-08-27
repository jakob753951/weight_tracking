FROM lukemathwalker/cargo-chef:latest AS chef
LABEL authors="jakob753951"

WORKDIR /weight_tracking_api

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /weight_tracking_api/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin weight_tracking_api

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm AS runtime
WORKDIR /weight_tracking_api
COPY --from=builder /weight_tracking_api/target/release/weight_tracking_api /usr/local/bin

RUN apt-get update
RUN apt-get install -y curl
RUN apt clean
RUN rm -rf /var/lib/apt/lists/*

HEALTHCHECK --start-period=30s --retries=5 --interval=10s --timeout=10s \
    CMD curl --fail http://localhost:7100/ping || exit 1

STOPSIGNAL sigkill

ENTRYPOINT ["/usr/local/bin/weight_tracking_api"]
