# (c) Copyright 2022 Christian Saide
# SPDX-License-Identifier: GPL-3.0

FROM --platform=${BUILDPLATFORM} rust:1-bullseye AS build

WORKDIR /opt/rift

ARG TARGETPLATFORM

COPY dist/bin/build-deps.sh /bin/build-deps.sh

RUN apt-get update \
    && apt-get upgrade -yqq \
    && bash /bin/build-deps.sh "${TARGETPLATFORM}" \
    && rm -rf /var/lib/apt/lists/*

# Copy in the Cargo configuration to fetch deps.
COPY Cargo.toml Cargo.lock .cargo/ ./
RUN cargo fetch

# Now copy in the source code to build and then compress the binary.
COPY . .

ARG TARGETARCH
ARG BUILD=debug

RUN --mount=type=cache,target=/root/.cargo \
    --mount=type=cache,sharing=private,target=/opt/rift/target \
    make BUILD=${BUILD} BUILD_ARCH=${TARGETARCH} compile

FROM --platform=${TARGETPLATFORM} gcr.io/distroless/static:latest AS runtime

ARG TARGETARCH
ARG BUILD=debug

LABEL org.opencontainers.image.source=https://github.com/rift-rs/rift
LABEL SPDX-License-Identifier=GPL-3.0
LABEL SPDX-License-Path=/LICENSE

HEALTHCHECK --interval=10s --timeout=1s --start-period=5s --retries=3 CMD [ "/bin/grpc_health_probe", "-addr", "localhost:8081" ]

USER nonroot
VOLUME [ "/var/lib/rift" ]
ENTRYPOINT [ "/bin/riftd" ]
CMD [ "--help" ]

COPY --from=build --chown=root:root --chmod=0644 /opt/rift/LICENSE /LICENSE
COPY --from=build --chown=nonroot:nonroot --chmod=0700 /opt/rift/grpc_health_probe /bin/grpc_health_probe
COPY --from=build --chown=nonroot:nonroot --chmod=0700 /opt/rift/output/${BUILD}/riftd_linux-${TARGETARCH} /bin/riftd
