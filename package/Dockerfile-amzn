FROM alpine
LABEL org.opencontainers.image.source "https://github.com/customink/crypteia"
LABEL org.opencontainers.image.description "Rust Lambda Extension for any Runtime to preload SSM Parameters as Secure Environment Variables!"

RUN mkdir -p /opt/lib
RUN mkdir -p /opt/extensions
COPY ./build/crypteia-amzn /opt/extensions/crypteia
COPY ./build/libcrypteia-amzn.so /opt/lib/libcrypteia.so
