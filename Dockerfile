FROM rust:latest
RUN mkdir -p /opt/tripledb
COPY . /opt/tripledb
WORKDIR /opt/tripledb
RUN cargo install
