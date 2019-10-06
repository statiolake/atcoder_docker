FROM ubuntu:18.04 as toolchain

MAINTAINER statiolake <statiolake@gmail.com>

USER root

### install toolchains

RUN sed -ie "s%http://archive.ubuntu.com/ubuntu/%http://ftp.jaist.ac.jp/pub/Linux/ubuntu/%g" /etc/apt/sources.list
RUN apt-get update
RUN apt-get install -y build-essential curl libssl-dev pkg-config git

# remove apt package index to reduce container size
RUN rm -rf /var/lib/apt/lists/*

ENV USER=root
ENV PATH=/root/.cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "1.38.0"

# remove documents to reduce container size
RUN rm -rf /root/.rustup/toolchains/*/share/doc

# prepare compile options resolver
FROM toolchain as tools

RUN cargo install --git https://github.com/rust-lang-ja/atcoder-rustc-dep-option-generator

# prepare a pre-compiled library
FROM toolchain as library

WORKDIR /

# clone project into libraries directory
RUN git clone https://github.com/rust-lang-ja/atcoder-rust-base --branch ja-all-enabled libraries

WORKDIR /libraries

# pre-compile crates
RUN cargo build --release

# run test
RUN cargo test -- --nocapture

# prepare compiler environment
FROM library as compiler

WORKDIR /

COPY --from=tools /root/.cargo/bin/rustc-dep-option-generator /root/.cargo/bin/rustc-dep-option-generator

RUN mkdir submission

WORKDIR /submission

ENTRYPOINT rustc --edition=2018 -C opt-level=3 -C target-cpu=native $(rustc-dep-option-generator /libraries/Cargo.toml /libraries/target/release/deps) main.rs && ./main < "in.txt"
