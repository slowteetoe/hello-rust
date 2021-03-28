FROM rust:1.51 as builder
LABEL maintainer="steven.lotito@alumni.pitt.edu"

WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=builder /app/target/release/hello-world /
EXPOSE 1111
CMD ["./hello-world"]