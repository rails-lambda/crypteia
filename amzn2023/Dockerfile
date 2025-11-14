FROM public.ecr.aws/amazonlinux/amazonlinux:2023

RUN dnf install -y gcc openssl-devel python3-pip
RUN dnf clean all

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN /root/.cargo/bin/rustup update \
    && /root/.cargo/bin/rustup target add aarch64-unknown-linux-gnu
RUN rustup default stable

ENV CRYPTEIA_BUILD_OS=amzn
ENV CRYPTEIA_BUILD_TARGET=x86_64-unknown-linux-gnu
