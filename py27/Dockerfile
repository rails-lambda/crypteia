FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt update && apt-get install -y \
    curl \
    gcc \
    libssl-dev \
    python3-pip \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN /root/.cargo/bin/rustup update \
    && /root/.cargo/bin/rustup target add aarch64-unknown-linux-gnu

RUN rustup default stable

WORKDIR /var/task

ENV CRYPTEIA_BUILD_OS=debian
ENV CRYPTEIA_BUILD_TARGET=x86_64-unknown-linux-gnu

