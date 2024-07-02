FROM rust
COPY . /app
WORKDIR /app
RUN cargo build --release
CMD target/release/commercyfy-core
