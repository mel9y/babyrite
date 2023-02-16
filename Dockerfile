FROM rust:slim-buster as build

COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

COPY --from=build /target/release/babyrite /

CMD [ "/babyrite" ]
