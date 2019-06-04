FROM ubuntu:18.04 as toolchain

MAINTAINER statiolake <statiolake@gmail.com>

USER root

### install toolchains

RUN apt-get update
RUN apt-get install -y build-essential curl

# remove apt package index to reduce container size
RUN rm -rf /var/lib/apt/lists/*

ENV USER=root
ENV PATH=/root/.cargo/bin:$PATH

ARG toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "${toolchain}"

# remove documents to reduce container size
RUN rm -rf /root/.rustup/toolchains/*/share/doc

# prepare a project
FROM toolchain as project

WORKDIR /

RUN cargo new submission

WORKDIR /submission

# import prepared Cargo.toml
ADD Cargo.toml /submission/Cargo.toml

# pre-compile crates
RUN cargo build --release

# remove current "hello world" binary
RUN cargo clean --release -p submission
RUN rm src/*.rs

ENV RUSTFLAGS='-C target-cpu=native'
ENTRYPOINT cargo run --release < "in.txt"
