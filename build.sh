#!/bin/sh

sudo docker build --tag statiolake/atcoder_sample --build-arg toolchain=nightly-2019-05-29 .
