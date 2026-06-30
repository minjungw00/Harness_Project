FROM rust:1.85-bookworm AS builder

WORKDIR /usr/src/volicord
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY tests ./tests
COPY xtask ./xtask

RUN cargo build --release -p volicord-cli --bin volicord

FROM debian:bookworm-slim AS runtime

RUN useradd --system --uid 10001 --create-home --home-dir /home/volicord volicord \
    && mkdir -p /var/lib/volicord /workspace \
    && chown -R volicord:volicord /var/lib/volicord /workspace

COPY --from=builder /usr/src/volicord/target/release/volicord /usr/local/bin/volicord

USER volicord
ENV VOLICORD_HOME=/var/lib/volicord
WORKDIR /workspace

ENTRYPOINT ["volicord"]
CMD ["--help"]
