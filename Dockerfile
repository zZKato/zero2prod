# Base image version
FROM rust:1.59-alpine3.15

# Switch working directory and install dependencies
WORKDIR /app
RUN apk --no-cache add lld clang musl-dev

# Copy code from repository to the docker container
COPY . .
# Compile with release flag
ENV SQLX_OFFLINE true
RUN cargo build --release

# Startup app
ENTRYPOINT ["./target/release/zero2prod"]