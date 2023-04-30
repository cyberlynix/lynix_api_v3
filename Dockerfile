FROM rust:latest

WORKDIR /lapi

COPY Cargo.toml Cargo.lock ./
# Optimize Build
RUN cargo build --release --locked
COPY src ./src


RUN cargo build --release --locked
EXPOSE 28300
CMD ["./target/release/lynix_api"]