FROM rust:1.51 as builder
LABEL maintainer="steven.lotito@alumni.pitt.edu"
#ENV PKG_CONFIG_ALLOW_CROSS=1

COPY . /usr/src/hello-world
WORKDIR /usr/src/hello-world
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10
COPY --from=builder /usr/local/cargo/bin/hello-world /usr/local/bin/hello-world
EXPOSE 1111
CMD ["hello-world"]