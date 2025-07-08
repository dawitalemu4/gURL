FROM dawitalemu4/gURL:latest AS builder


FROM rust:1.88

RUN apt-get update && apt-get install -y grpcurl

COPY --from=builder /gURL /gURL
COPY --from=builder /public /public
COPY --from=builder /templates /templates 

COPY .env .

CMD ["/gURL"]
