FROM rust as build
ARG DB_USER
ARG DB_PASSWORD
ARG DB_HOST
ARG DB_NAME
ARG DISCORD_HOOK_URL
ENV DB_USER $DB_USER
ENV DB_PASSWORD $DB_PASSWORD
ENV DB_HOST $DB_HOST
ENV DB_NAME $DB_NAME
ENV DISCORD_HOOK_URL $DISCORD_HOOK_URL
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./static ./static

RUN cargo build --release

FROM debian:bookworm-slim

RUN mkdir app
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev

COPY --from=build /app/target/release ./
COPY --from=build /app/static ./static

EXPOSE 8080

ENTRYPOINT ["/app/actix-website"]
