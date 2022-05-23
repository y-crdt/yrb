FROM ruby:3.0.3

############################################################
# dependencies
############################################################

RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    clang \
    cmake \
    gcc \
    g++ \
    git \
    zlib1g-dev \
    libmpc-dev \
    libmpfr-dev \
    libgmp-dev \
    ca-certificates \
    curl \
    file \
    build-essential \
    autoconf \
    automake \
    autotools-dev \
    libtool \
    xutils-dev && \
    rm -rf /var/lib/apt/lists/*

ENV SSL_VERSION=1.0.2u

RUN curl https://www.openssl.org/source/openssl-$SSL_VERSION.tar.gz -O && \
    tar -xzf openssl-$SSL_VERSION.tar.gz && \
    cd openssl-$SSL_VERSION && ./config && make depend && make install && \
    cd .. && rm -rf openssl-$SSL_VERSION*

ENV OPENSSL_LIB_DIR=/usr/local/ssl/lib \
    OPENSSL_INCLUDE_DIR=/usr/local/ssl/include \
    OPENSSL_STATIC=1

############################################################
# WORKDIR
############################################################

WORKDIR /root

############################################################
# Rust
############################################################

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH

# the .cargo file contains the required linker config
COPY config/build/.cargo ./

############################################################
# add all supported targets
############################################################

# windows
RUN rustup target add x86_64-pc-windows-gnu

# linux
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add aarch64-unknown-linux-gnu

# macos
RUN rustup target add x86_64-apple-darwin
RUN rustup target add aarch64-apple-darwin

############################################################
# Cross-compile
############################################################

RUN git clone https://github.com/tpoechtrager/osxcross && \
  cd osxcross && \
  wget -nc https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz && \
  mv MacOSX10.10.sdk.tar.xz tarballs/ && \
  UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh

ENV PATH="$(pwd)/osxcross/target/bin:$PATH"

############################################################
# fetch dependencies to speedup builds
############################################################

COPY Cargo.lock Cargo.toml ./
COPY src/ ./src

RUN cargo fetch
