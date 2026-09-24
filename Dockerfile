# Stage 1

FROM rust:1.97-alpine AS build
WORKDIR /app
RUN apk add --no-cache musl-dev ca-certificates
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src target/release/deps/bachan* target/release/bachan*
COPY src src
COPY migrations migrations
RUN cargo build --release
COPY . .

# Stage 2

FROM alpine:3.19
WORKDIR /app
RUN apk --no-cache add ca-certificates
COPY --from=build /app/target/release/bachan .
COPY --from=build /app/templates templates
COPY --from=build /app/static static
EXPOSE 3000
ENTRYPOINT ["./bachan"]