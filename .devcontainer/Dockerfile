FROM mcr.microsoft.com/vscode/devcontainers/rust

RUN sudo apt-get update -y \
    && sudo apt-get upgrade -y \
    && sudo apt-get install -y zip

RUN rustup update \
    && rustup target add x86_64-unknown-linux-gnu \
    && rustup target add aarch64-unknown-linux-gnu

# Easy way to install node, ruby, and php
RUN apt-get -y install nodejs ruby php php-common

# Easy way to install Python.
RUN update-alternatives --install /usr/bin/python python /usr/bin/python3 1

# Multi-platform SAM CLI. https://github.com/aws/aws-sam-cli/issues/3908
RUN apt-get install -y pip && pip install aws-sam-cli
