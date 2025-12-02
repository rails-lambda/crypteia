FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

RUN apt-get update -y \
    && apt-get upgrade -y \
    && apt-get install -y --fix-missing --no-install-recommends \
    \
    # Zip for packaging
    zip \
    \
    # QEMU for multi-architecture support
    qemu-system \
    binfmt-support \
    qemu-user-static \
    \
    # Language runtimes for tests
    nodejs \
    ruby \
    php \
    php-common \
    python3-pip \
    \
    # Cross-compilation toolchains
    gcc-x86-64-linux-gnu \
    libc6-dev-amd64-cross \
    gcc-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    \
    # Clean up, enable QEMU, and set Python alternative in a single layer
    && rm -rf /var/lib/apt/lists/* \
    && update-binfmts --enable qemu-aarch64 \
    && update-alternatives --install /usr/bin/python python /usr/bin/python3 1

# Switch to root to install rust targets and fix permissions
USER root

# Install rust targets for cross-compilation
RUN rustup update \
    && rustup default stable \
    && rustup target add aarch64-unknown-linux-gnu \
    && rustup target add x86_64-unknown-linux-gnu

# Grant vscode user ownership of rustup and cargo directories
RUN chown -R vscode:vscode /usr/local/rustup /usr/local/cargo

# Switch back to vscode user
USER vscode

# Multi-platform SAM CLI. https://github.com/aws/aws-sam-cli/issues/3908
RUN pip install aws-sam-cli --break-system-packages
