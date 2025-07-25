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


# FROM debian:stable-20250721-slim

# RUN apt-get update && \
#     apt-get install -y curl && \
#     rm -rf /var/lib/apt/lists/*

# RUN curl -L https://github.com/fullstorydev/grpcurl/releases/download/v1.9.3/grpcurl_1.9.3_linux_x86_64.tar.gz -o /tmp/grpcurl.tar.gz && \
#     tar -xzf /tmp/grpcurl.tar.gz -C /usr/local/bin/

# RUN apt purge -y curl

# COPY --from=builder /app/target/release/gURL /usr/local/bin/gURL
# COPY --from=builder /app/init.sql /init.sql
# COPY --from=builder /app/public /public

# docker buildx build --platform linux/amd64,linux/arm64,linux/arm/v7 -t dawitalemu4/gurl:latest --push .



# for me (test locally)

FROM rust:${RUST_VERSION} AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --features docker


FROM debian:stable-20250721-slim

RUN apt-get update && \
    apt-get install -y curl && \
    rm -rf /var/lib/apt/lists/*

RUN curl -L https://github.com/fullstorydev/grpcurl/releases/download/v1.9.3/grpcurl_1.9.3_linux_x86_64.tar.gz -o /tmp/grpcurl.tar.gz && \ 
   tar -xzf /tmp/grpcurl.tar.gz -C /usr/local/bin/

RUN apt purge -y curl

COPY --from=builder /app/target/release/gURL /usr/local/bin/gURL
COPY --from=builder /app/init.sql /init.sql
COPY --from=builder /app/public /public

COPY .env .

CMD ["gURL", "--features", "docker"]
