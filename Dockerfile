FROM rust:latest

# Workdir
WORKDIR /lapi

# Copy Files
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build
RUN cargo build --release --locked

# Run
EXPOSE 28300
CMD ["./target/release/lynix_api"]