# for you

# FROM dawitalemu4/gURL:latest AS builder


# FROM rust:1.88

# RUN apt-get update && apt-get install -y grpcurl

# COPY --from=builder /gURL /gURL
# COPY --from=builder /go/views /go/views
# COPY --from=builder /templates /templates 

# COPY .env .

# CMD ["/gURL"]


# for me (push to docker hub)

# FROM rust:1.88 AS builder

# COPY . .

# RUN cargo build -o /gURL

# docker image build -t gURL .
# docker image tag gURL dawitalemu4/gURL:latest
# docker push dawitalemu4/gURL:latest


# for me (test locally)

FROM rust:1.88 AS builder

COPY . .

RUN cargo build -o /gURL


FROM rust:1.88

RUN apt-get update && apt-get install -y grpcurl

COPY --from=builder /gURL /gURL
COPY --from=builder /go/views /go/views
COPY --from=builder /templates /templates 

COPY .env .

CMD ["/gURL"]


# for me (test published image)

# FROM dawitalemu4/gURL:latest AS builder


# # change the next line to FROM --platform=linux/amd64 rust:1.88 if you are a mac user and getting this error: "rosetta error: failed to open elf at /lib64/ld-linux-x86-64.so.2"
# FROM rust:1.88

# RUN apt-get update && apt-get install -y grpcurl

# COPY --from=builder /gURL /gURL
# COPY --from=builder /go/views /go/views
# COPY --from=builder /templates /templates 

# COPY .env .

# CMD ["/gURL"]
