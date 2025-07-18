ARG RUST_VERSION=1.88

# for you

# FROM dawitalemu4/gurl:latest

# COPY .env .

# CMD ["gURL", "--features", "docker"]



# for me (push to docker hub)

# FROM rust:${RUST_VERSION} AS builder

# WORKDIR /app

# COPY . .

# RUN cargo build --release --features docker


# FROM archlinux:base

# RUN curl -L https://github.com/fullstorydev/grpcurl/releases/download/v1.9.3/grpcurl_1.9.3_linux_x86_64.tar.gz -o /tmp/grpcurl.tar.gz && \
#     tar -xzf /tmp/grpcurl.tar.gz -C /usr/local/bin/

# COPY --from=builder /app/target/release/gURL /usr/local/bin/gURL
# COPY --from=builder /app/init.sql /init.sql
# COPY --from=builder /app/public /public

# docker image build -t gurl .
# docker image tag gurl dawitalemu4/gurl:latest
# docker push dawitalemu4/gurl:latest



# for me (test image before publish)

# FROM gurl:latest

# COPY .env .

# CMD ["gURL", "--features", "docker"]



# for me (test locally)

FROM rust:${RUST_VERSION} AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --features docker


FROM archlinux:base

RUN curl -L https://github.com/fullstorydev/grpcurl/releases/download/v1.9.3/grpcurl_1.9.3_linux_x86_64.tar.gz -o /tmp/grpcurl.tar.gz && \ 
   tar -xzf /tmp/grpcurl.tar.gz -C /usr/local/bin/

COPY --from=builder /app/target/release/gURL /usr/local/bin/gURL
COPY --from=builder /app/init.sql /init.sql
COPY --from=builder /app/public /public

COPY .env .

CMD ["gURL", "--features", "docker"]
