# Auto Waifu Generator

## Why

Because I was lazy and wanted to an automated way to keep my discord bot "special" with new avatars per day

## Cross Compiling

This project uses [Cross](https://github.com/rust-embedded/cross) to cross compile to raspberry pi. First install it by running

```
cargo install cross
```

Once installed you can call the helper build scrip to produce a binary compatable for rpi3/4

```
./scripts/cross-build.sh
```
