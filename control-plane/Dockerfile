FROM rust:1.95-bookworm AS build
WORKDIR /src
COPY control-plane ./control-plane
WORKDIR /src/control-plane
RUN cargo build --locked --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl     && rm -rf /var/lib/apt/lists/*
RUN useradd --create-home --uid 10001 gitgat
COPY --from=build /src/control-plane/target/release/gitgat-control-plane /usr/local/bin/gitgat-control-plane
USER gitgat
ENV GITGAT_BIND=0.0.0.0:8080
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=5s --retries=3 CMD curl -fsS http://127.0.0.1:8080/health || exit 1
ENTRYPOINT ["/usr/local/bin/gitgat-control-plane"]
