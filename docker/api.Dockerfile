FROM rust:1.82-slim AS build
WORKDIR /app

# Install minimal build deps (git sometimes needed for crates; ca-certificates for TLS)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates pkg-config \
  && rm -rf /var/lib/apt/lists/*

COPY engine ./engine
COPY api ./api

WORKDIR /app/api
RUN cargo build --release

# Distroless runtime (small attack surface)
FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
COPY --from=build /app/api/target/release/policygate-api /app/policygate-api

EXPOSE 8080
ENTRYPOINT ["/app/policygate-api"]
