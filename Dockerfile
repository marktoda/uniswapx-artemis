# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.80.1
ARG APP_NAME=uniswapx-artemis

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-bookworm AS build
ARG APP_NAME
WORKDIR /app


# AWS CodeBuild doesn't seem to support buildkit so can't use --mount
COPY . .
RUN cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/server

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
FROM debian:bookworm-slim AS final
RUN apt-get -y update && apt-get -y upgrade && apt-get install -y libssl3 ca-certificates && update-ca-certificates && rm -rf /var/lib/apt/lists/*

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 1559
