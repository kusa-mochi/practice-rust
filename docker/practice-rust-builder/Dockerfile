FROM rust:alpine3.19

RUN addgroup mochirust && \
    adduser -h /home/mochirust -D mochirust -G mochirust && \
    chown -R mochirust:mochirust /home/mochirust && \
    chmod -R 774 /home/mochirust
USER mochirust

WORKDIR /home/mochirust/
