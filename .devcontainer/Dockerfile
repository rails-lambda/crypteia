FROM mcr.microsoft.com/vscode/devcontainers/rust

RUN sudo apt-get update -y \
    && sudo apt-get upgrade -y \
    && sudo apt-get install zip musl-tools ltrace -y

RUN rustup update \
    && rustup target add x86_64-unknown-linux-musl

RUN sudo apt-get install -y nodejs

RUN cd /tmp \
    && curl -L https://github.com/aws/aws-sam-cli/releases/latest/download/aws-sam-cli-linux-x86_64.zip > aws-sam-cli-linux-x86_64.zip \
    && unzip aws-sam-cli-linux-x86_64.zip -d sam-installation \
    && rm -rf aws-sam-cli-linux-x86_64.zip \
    && sudo ./sam-installation/install
