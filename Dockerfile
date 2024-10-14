# build container
FROM rust:slim-bullseye AS builder
RUN apt update && apt install -y librust-openssl-dev libssl-dev
RUN mkdir /app
COPY . /app
RUN cd /app && cargo build --release

# target container
FROM rust:slim-bullseye
ENV TZ=Asia/Tokyo
RUN mkdir /app
COPY --from=builder /app/target/release/simple-restaurant-api /app/simple-restaurant-api
COPY .env /app/.env
WORKDIR /app
CMD ["./simple-restaurant-api"]