FROM debian:bullseye-slim

ARG TARGET

RUN mkdir -p /usr/memorial

COPY target/${TARGET}/release/memorial-cli /usr/memorial

RUN useradd --user-group --create-home --no-log-init --shell /bin/bash memorial

RUN chown -R memorial:memorial /usr/memorial

USER memorial

WORKDIR /src

ENTRYPOINT [ "/usr/memorial/memorial-cli", "scan" ]
