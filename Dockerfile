FROM rust:slim-buster as build

COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

LABEL org.opencontainers.image.source=="https://github.com/m2en/babyrite"

COPY --from=build /target/release/babyrite /

CMD [ "/babyrite" ]
