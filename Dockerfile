FROM ekidd/rust-musl-builder:latest as build

# Add the source code (+fix file permissions)
ADD --chown=rust:rust src src
ADD --chown=rust:rust Cargo.toml Cargo.toml

# Build
RUN cargo build --release



FROM scratch

COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/ .