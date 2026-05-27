# ------------------------------------------------------------
# - 1. The Chef: Installs cargo-chef 
# ------------------------------------------------------------

FROM rust:1.95.0-slim-bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# ------------------------------------------------------------
# - 2. The planner: Computes the recipe in a lock-like file
# ------------------------------------------------------------

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ------------------------------------------------------------
# - 3. The builder: Builds the application
# ------------------------------------------------------------
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Step A: Build dependencies ONLY
# This layer is heavily cached. As long as your Cargo.toml/lock
# don't change, Docker skips this step
RUN cargo chef cook --release --recipe-path recipe.json

# Step B: Build the application
# Now we copy in our actual source code
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin zero2prod

# ------------------------------------------------------------
# - 4. The runtime: Runs the application
# ------------------------------------------------------------

FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify the TLS certificates
# when establishning HTTP connections

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/zero2prod zero2prod
#We need the configuration file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./zero2prod"]