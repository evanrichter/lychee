# Build Stage
FROM rustlang/rust:nightly as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN rm Cargo.toml && \
    cd lychee-bin && \
    env RUSTFLAGS="-g -Cdebug-assertions=on" \
        cargo +nightly -Z sparse-registry build --release && \
    mv target/release/lychee /lychee

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /lychee /
