FROM rust:1.84 as builder

WORKDIR /usr/src/belle

COPY . .

RUN chmod +x build.sh && ./build.sh -n
RUN cd btils && gcc -o bfmt bfmt.c -static && cp bfmt ../bin && cd ..
RUN cd bdump && make release && cp bdump ../bin && cd ..

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /usr/src/belle/bin/belle ./
COPY --from=builder /usr/src/belle/bin/basm ./

RUN chmod +x ./belle ./basm

ENTRYPOINT ["./belle"]
