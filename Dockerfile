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

### prepare compile options resolver

FROM toolchain as tools

ADD gen-deps-compile-options /gen-deps-compile-options
WORKDIR /gen-deps-compile-options
RUN cargo build --release

ADD target_features /target_features

ADD library-build-tool /library-build-tool
WORKDIR /library-build-tool
RUN cargo build --release

ADD find-proj-dir-by-feature /find-proj-dir-by-feature
WORKDIR /find-proj-dir-by-feature
RUN cargo build --release

### prepare a pre-compiled library

FROM toolchain as library

# create each precompiled libraries under /libraries
RUN mkdir /libraries

WORKDIR /libraries

# import prepared Cargo.toml
ADD Cargo.toml.skel /libraries/Cargo.toml.skel
ADD main.rs.skel /libraries/main.rs.skel
ADD target_features /target_features

# pre-build libraries
COPY --from=tools /library-build-tool/target/release/library-build-tool /root/.cargo/bin/library-build-tool
RUN library-build-tool /libraries/Cargo.toml.skel /libraries/main.rs.skel /target_features /libraries

# prepare compiler environment
FROM library as compiler

WORKDIR /

COPY --from=tools /gen-deps-compile-options/target/release/gen-deps-compile-options /root/.cargo/bin/gen-deps-compile-options
COPY --from=tools /find-proj-dir-by-feature/target/release/find-proj-dir-by-feature /root/.cargo/bin/find-proj-dir-by-feature
RUN mkdir submission

WORKDIR /submission

ENTRYPOINT rustc --edition=2018 -C opt_level=3 -C target_cpu=native \
    $(gen-deps-compile-options "$(find-proj-dir-by-feature /libraries)/Cargo.toml" "$(find-proj-dir-by-feature /libraries)/target/release/deps") main.rs \
    && ./main < "in.txt"
