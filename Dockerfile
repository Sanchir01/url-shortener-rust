FROM messense/rust-musl-cross:x86_64-musl AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.20 AS runner

RUN apk add --no-cache libgcc

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/url-shortener ./url-shortener

COPY --from=builder /app/src/config/prod.toml ./

ENTRYPOINT ["/url-shortener"]

EXPOSE 4200
