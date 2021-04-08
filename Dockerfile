FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust
WORKDIR /run
COPY --from=builder /app/target/release/shiber .
ENTRYPOINT [ "./shiber" ]