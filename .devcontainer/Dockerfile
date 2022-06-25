FROM mcr.microsoft.com/vscode/devcontainers/rust

RUN sudo apt-get update -y && \
    sudo apt-get upgrade -y && \
    sudo apt-get install zip musl-tools -y

RUN rustup update && \
    rustup target add x86_64-unknown-linux-musl

RUN sudo apt-get install -y nodejs
