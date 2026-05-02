# Build stage
FROM rust:1.95-slim AS builder
WORKDIR /app
COPY . .
# Use SQLx offline mode for Docker builds
COPY .sqlx .sqlx
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/wheelbase-backend /usr/local/bin/app
EXPOSE 8080
CMD ["/usr/local/bin/app"]
