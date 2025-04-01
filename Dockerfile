#FROM ubuntu:latest
#LABEL authors="chukwuebuka"
#
#ENTRYPOINT ["top", "-b"]


#FROM rust:1.80 AS builder
#WORKDIR /usr/src/app
#COPY . .
#RUN cargo build --release
#
#FROM debian:buster-slim
#RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
#COPY --from=builder /usr/src/app/target/release/ev_charging_infra /usr/local/bin/
#CMD ["ev_charging_infra"]



# Use Rust nightly
FROM rustlang/rust:nightly AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/ev_charging_infra /usr/local/bin/
CMD ["ev_charging_infra"]