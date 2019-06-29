FROM ubuntu:18.04 as toolchain

MAINTAINER statiolake <statiolake@gmail.com>

USER root

### install toolchains

RUN apt-get update
RUN apt-get install -y build-essential curl libssl-dev pkg-config

# remove apt package index to reduce container size
RUN rm -rf /var/lib/apt/lists/*

ENV USER=root
ENV PATH=/root/.cargo/bin:$PATH

ARG toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "${toolchain}"

# remove documents to reduce container size
RUN rm -rf /root/.rustup/toolchains/*/share/doc

# prepare compile options resolver
FROM toolchain as tools

RUN cargo install --git https://github.com/rust-lang-ja/atcoder-rustc-dep-option-generator --branch gitrev

# prepare a pre-compiled library
FROM toolchain as library

WORKDIR /

RUN cargo new libraries

WORKDIR /libraries

# import prepared Cargo.toml
ADD Cargo.toml.skel /libraries/Cargo.toml

# pre-compile crates
RUN cargo build --release

# prepare compiler environment
FROM library as compiler

WORKDIR /

COPY --from=tools /root/.cargo/bin/rustc-dep-option-generator /root/.cargo/bin/rustc-dep-option-generator

RUN mkdir submission

WORKDIR /submission

ENTRYPOINT rustc --edition=2018 -C opt-level=3 -C target-cpu=native $(rustc-dep-option-generator /libraries/Cargo.toml /libraries/target/release/deps) main.rs && ./main < "in.txt"
