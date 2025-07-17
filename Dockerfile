FROM messense/rust-musl-cross:x86_64-musl AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo 'fn main() {}' > src/main.rs

RUN cargo build --release --target x86_64-unknown-linux-musl

RUN rm -rf src

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.20 AS runner

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/url-shortener ./url-shortener

COPY --from=builder /app//src/config/prod.toml ./

EXPOSE 4200

CMD ["./url-shortener"]