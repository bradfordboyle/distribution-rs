FROM debian:testing

RUN apt-get update && \
    apt-get install --yes \
    binutils-dev \
    cmake \
    curl \
    g++ \
    gcc \
    libcurl4-openssl-dev \
    libdw-dev \
    libiberty-dev \
    pkgconf \
    python \
    zlib1g-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN curl --progress --retry 3 --retry-delay 15 -L https://github.com/SimonKagstrom/kcov/archive/v34.tar.gz -o kcov-v34.tar.gz && \
tar xzf kcov-v34.tar.gz && \
mkdir kcov-34/build && \
cd kcov-34/build && \
cmake .. && \
make && \
make install
