FROM scorpil/rust:1.16
MAINTAINER Jonathan Reyes <j@jreyes.org>

WORKDIR /usr/local/src/app
RUN apt-get -qq update && apt-get -qq install --no-install-recommends \
        libssl-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock /usr/local/src/app/
RUN cargo fetch
COPY . /usr/local/src/app/
RUN cargo build --release && \
    mv target/release/prensa-corrupta . && \
    rm -rf target

CMD ["./prensa-corrupta"]
