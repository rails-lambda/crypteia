FROM public.ecr.aws/sam/build-nodejs16.x

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="~/.cargo/bin:${PATH}"
ENV BUILD_ARCH=amzn
