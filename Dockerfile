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

ADD gen-deps-compile-options /gen-deps-compile-options

WORKDIR /gen-deps-compile-options

RUN cargo build --release

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

COPY --from=tools /gen-deps-compile-options/target/release/gen-deps-compile-options /root/.cargo/bin/gen-deps-compile-options

RUN mkdir submission

WORKDIR /submission

ENV RUSTFLAGS='-C target-cpu=native'
ENTRYPOINT rustc --edition=2018 -C opt_level=3 $(gen-deps-compile-options /libraries/Cargo.toml /libraries/target/release/deps) main.rs && ./main < "in.txt"
