FROM rust:latest

LABEL maintainer="KMNK<kmnknmk+com-github@gmail.com>"

ENV USER="KMNK<kmnknmk+com-github@gmail.com>"
ENV PROJECT="hello_world"

WORKDIR projects

CMD ["/bin/sh", "-c", "cd $PROJECT && cargo run"]