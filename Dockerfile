ARG BASE_IMAGE=debian:bookworm-slim

FROM ${BASE_IMAGE} as builder
ARG RUST_VERSION=1.75.0
SHELL ["/bin/bash", "-c"]

WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y libssl-dev pkg-config build-essential cmake curl libpaho-mqtt-dev dbus libdbus-1-dev
RUN curl --proto '=https' -sSf https://sh.rustup.rs/ | bash -s -- --default-toolchain=${RUST_VERSION} -y
RUN source "/$HOME/.cargo/env" && \
  cargo build -r

FROM ${BASE_IMAGE} as runner
ARG APP

RUN apt-get update && \
  apt-get install -y libssl-dev ca-certificates dbus libdbus-1-dev  && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/${APP} /app/
#COPY static /static

RUN echo "#!/bin/bash" > /entrypoint.sh
RUN echo "/app/${APP}" >> /entrypoint.sh
RUN chmod +x /entrypoint.sh

CMD ["/entrypoint.sh"]

