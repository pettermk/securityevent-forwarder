FROM rust:1.66 as build-env
WORKDIR /app
# COPY . /app
COPY src/ src/
COPY migrations/ migrations/
COPY *.toml ./
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y postgresql && rm -rf /var/lib/apt/lists/*
COPY --from=build-env /usr/local/cargo/bin/snyk-webhook-api /usr/local/bin/snyk-webhook-api
CMD ["snyk-webhook-api"]
