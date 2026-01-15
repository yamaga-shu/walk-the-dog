FROM "mcr.microsoft.com/devcontainers/rust:2-1-trixie"

# install crates
RUN cargo install --locked trunk

# install wasm target
RUN rustup target add wasm32-unknown-unknown
