FROM mcr.microsoft.com/vscode/devcontainers/rust

RUN sudo apt-get update -y \
    && sudo apt-get upgrade -y \
    && sudo apt-get install -y zip ltrace

RUN rustup update \
    && rustup target add aarch64-unknown-linux-gnu

RUN sudo apt-get install -y \
    qemu \
    binfmt-support \
    qemu-user-static

RUN cd /tmp \
    && curl -L https://github.com/aws/aws-sam-cli/releases/latest/download/aws-sam-cli-linux-x86_64.zip > aws-sam-cli-linux-x86_64.zip \
    && unzip aws-sam-cli-linux-x86_64.zip -d sam-installation \
    && rm -rf aws-sam-cli-linux-x86_64.zip \
    && sudo ./sam-installation/install
