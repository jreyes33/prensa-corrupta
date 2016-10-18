FROM jreyes33/rust:nightly
MAINTAINER Jonathan Reyes <j@jreyes.org>

WORKDIR /usr/local/src/app
COPY Cargo.toml Cargo.lock /usr/local/src/app/
RUN cargo fetch
COPY . /usr/local/src/app/
RUN cargo build --release && \
    mv target/release/prensa-corrupta . && \
    rm -rf target

CMD ["./prensa-corrupta"]
