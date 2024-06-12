FROM public.ecr.aws/sam/build-nodejs18.x

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="~/.cargo/bin:${PATH}"

RUN ~/.cargo/bin/rustup update \
    && ~/.cargo/bin/rustup target add aarch64-unknown-linux-gnu

RUN rustup default stable
ENV CRYPTEIA_BUILD_OS=amzn
ENV CRYPTEIA_BUILD_TARGET=x86_64-unknown-linux-gnu
