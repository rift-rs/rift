#!/bin/bash

# (c) Copyright 2022 Christian Saide
# SPDX-License-Identifier: GPL-3.0g

BUILD_DEPS="upx-ucl"
PROBE_VERSION="v0.4.6"

function amd64() {
    apt-get install -yqq gcc ${BUILD_DEPS}
    rustup target add x86_64-unknown-linux-gnu
    curl -L https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${PROBE_VERSION}/grpc_health_probe-linux-amd64 --output grpc_health_probe
}

function arm64() {
    apt-get install -yqq gcc-aarch64-linux-gnu ${BUILD_DEPS}
    rustup target add aarch64-unknown-linux-gnu
    curl -L https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${PROBE_VERSION}/grpc_health_probe-linux-arm64 --output grpc_health_probe
}

case "${1}" in
    "linux/arm64") arm64 ;;
    "linux/amd64") amd64 ;;
    *) exit 1 ;;
esac

rustup component add rustfmt