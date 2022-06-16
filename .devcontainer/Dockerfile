# rustup 1.24.3 rustc 1.60.0
FROM mcr.microsoft.com/vscode/devcontainers/rust:0.202.0-1

RUN sudo apt-get update -y && \
    sudo apt-get upgrade -y && \
    sudo apt-get install zip musl-tools -y

RUN rustup update && \
    rustup target add x86_64-unknown-linux-musl

RUN cd /tmp && \
    curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" && \
    unzip awscliv2.zip && \
    sudo ./aws/install && \
    rm -rf /tmp/aws*
