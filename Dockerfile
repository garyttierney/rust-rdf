FROM alpine:latest
RUN apk update
RUN apk add ca-certificates wget && update-ca-certificates
RUN apk add llvm-libunwind libstdc++


RUN mkdir -p /opt/
COPY . /opt/tripledb
WORKDIR /opt

RUN apk add --virtual .build-rust g++ gcc make rust cargo snappy-dev zlib-dev bzip2-dev lz4-dev linux-headers && \
    wget http://github.com/facebook/zstd/archive/v1.1.3.tar.gz && \
    mv v1.1.3.tar.gz zstd-1.1.3.tar.gz && \
    tar zxvf zstd-1.1.3.tar.gz && \
    cd zstd-1.1.3 && \
    make -j4 && make install && \
    cd /opt/tripledb && \
    cargo build -q --release && \
    cp target/release/tripledb . && \
    rm -rf target/ ~/.cargo/ && \
    apk del --purge .build-rust

WORKDIR /opt/tripledb
ENTRYPOINT ["./tripledb"]
