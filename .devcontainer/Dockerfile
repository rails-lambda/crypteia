FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN sudo apt-get update -y \
    && sudo apt-get upgrade -y 

RUN sudo apt-get install -y --fix-missing zip 

RUN sudo apt-get update -y \
    && sudo apt-get upgrade -y \
    && sudo apt-get install -y zip ltrace

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN rustup update \
    && rustup target add aarch64-unknown-linux-gnu

RUN rustup default stable
# x86_64 to arm64 support.
RUN sudo apt-get install -y \
    qemu \
    binfmt-support \
    qemu-user-static

# Easy way to install node, ruby, and php
RUN apt-get -y install nodejs ruby php php-common

# Easy way to install Python.
RUN update-alternatives --install /usr/bin/python python /usr/bin/python3 1

# Multi-platform SAM CLI. https://github.com/aws/aws-sam-cli/issues/3908
RUN apt-get install -y pip && pip install aws-sam-cli
