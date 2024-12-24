FROM rust:1.83 as builder

WORKDIR /usr/src/belle

COPY . .

RUN chmod +x build.sh && ./build.sh

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /usr/src/belle/bin/belle ./
COPY --from=builder /usr/src/belle/bin/basm ./

RUN chmod +x ./belle ./basm

ENTRYPOINT ["./belle"]
