FROM alpine

LABEL org.opencontainers.image.source=https://github.com/Kostassoid/memorial
LABEL org.opencontainers.image.description="A CLI tool for collecting notes from the source code files"
LABEL org.opencontainers.image.licenses=MIT

COPY memorial-cli /bin/

WORKDIR /project

ENTRYPOINT [ "/bin/memorial-cli" ]
