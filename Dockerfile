FROM raspbian/jessie

# See the official Rust dockerfile - inspiration has been taken from there:
# https://github.com/rust-lang/docker-rust/blob/76921dd61d80c4e8107b858d26bf5e52c4c09816/1.41.0/stretch/Dockerfile
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.41.0

RUN wget https://static.rust-lang.org/rustup/archive/1.21.1/armv7-unknown-linux-gnueabihf/rustup-init; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN apt-get update;
RUN apt-get -y install build-essential libsqlite3-dev;

RUN mkdir /app;

WORKDIR /app

VOLUME [ "/app" ]
