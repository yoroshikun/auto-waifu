
FROM alpine:latest
COPY ./target/x86_64-unknown-linux-musl/release/auto-waifu /auto-waifu
ENTRYPOINT ["/auto-waifu"]