ARG RUST_VERSION=1.88

# for you

# FROM dawitalemu4/gurl:latest AS builder


# FROM rust:${RUST_VERSION}

# RUN snap install grpcurl

# COPY --from=builder /gURL/.target/release/gURL /gURL
# COPY --from=builder /public /public
# COPY --from=builder /templates /templates 

# COPY .env .

# CMD ["/gURL"]


# for me (push to docker hub)

# FROM rust:${RUST_VERSION} AS builder

# COPY . .

# RUN cargo build --release --target-dir /gURL

# docker image build -t gurl .
# docker image tag gurl dawitalemu4/gurl:latest
# docker push dawitalemu4/gurl:latest


# for me (test locally)

FROM rust:${RUST_VERSION} AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --features docker


FROM archlinux:base

RUN pacman -Syu --noconfirm curl && \
    curl -L https://github.com/fullstorydev/grpcurl/releases/download/v1.9.3/grpcurl_1.9.3_linux_x86_64.tar.gz -o /tmp/grpcurl.tar.gz && \
    tar -xzf /tmp/grpcurl.tar.gz -C /tmp && \
    mv /tmp/grpcurl /usr/local/bin/ && \
    chmod +x /usr/local/bin/grpcurl && \
    rm /tmp/grpcurl.tar.gz

COPY --from=builder /app/target/release/gURL /usr/local/bin/gURL
COPY --from=builder /app/init.sql /init.sql 

COPY .env .

CMD ["gURL", "--features", "docker"]


# for me (test published image)

# FROM dawitalemu4/gurl:latest AS builder


# FROM rust:${RUST_VERSION}

# RUN snap install grpcurl

# COPY --from=builder /gURL/.target/release/gURL /gURL
# COPY --from=builder /public /public
# COPY --from=builder /templates /templates 

# COPY .env .

# CMD ["/gURL"]
