FROM debian:latest
MAINTAINER Rob Gries <robert.w.gries@gmail.com>

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    build-essential \
    curl \
    qemu-system \
    git \
    libssl-dev \
    pkg-config

RUN useradd --create-home --shell /bin/bash rxinu
USER rxinu
WORKDIR /home/rxinu

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH="$PATH:/$HOME/.cargo/bin" && \
    rustup default nightly && \
    rustup component add rust-src && \
    rustup component add llvm-tools-preview && \
    cargo install bootimage && \
    rustup component add rustfmt --toolchain nightly

ENV PATH="$PATH:/home/rxinu/.cargo/bin"
WORKDIR /home/rxinu/rxinu
CMD /bin/bash