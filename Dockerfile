FROM debian:jessie
MAINTAINER Rob Gries <robert.w.gries@gmail.com>

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        build-essential \
        nasm \
        grub-common \
        grub-pc-bin \
        xorriso \
        curl \
        git;
 
RUN useradd --create-home --shell /bin/bash rxinu
USER rxinu
WORKDIR /home/rxinu
ENTRYPOINT ["/bin/bash"]

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y;

RUN export PATH=$PATH:$HOME/.cargo/bin; \
    rustup component add rust-src; \
    cargo install xargo;
