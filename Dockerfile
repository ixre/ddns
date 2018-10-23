# jarry's short address service images
# Version : 0.1
# Author : jarrysix(jarrysix@gmail.com)
# Date : 2018-10-21 22:39

FROM ekidd/rust-musl-builder AS builder
# Add our source code.
ADD . ./
# Fix permissions on source code and Build our application.
RUN sudo chown -R rust:rust ../ && cargo build --release

FROM alpine:latest
MAINTAINER jarrysix
LABEL Vendor="github.com/jsix"
LABEL License="GPLv2"
LABEL Version=1.0.0

WORKDIR /usr/bin
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/ddns \
    /usr/bin/ddns

VOLUME ["/ddns"]
EXPOSE 8888
ENTRYPOINT ddns -c /jsa



