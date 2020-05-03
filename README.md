# Auto Waifu Generator

## Why

Because I was lazy and wanted to an automated way to keep my discord bot "special" with new avatars per day

## Requirements

The host system must have chromeium installed & and or chromedriver

```bash
# Mac
brew cask install chromedriver

# Linux RPi
apt install chromium-chromedriver
```

## Cross Compiling

This project uses [Cross](https://github.com/rust-embedded/cross) to cross compile to raspberry pi. First install it by running

```
cargo install cross
```

Once installed you can call the helper build script to produce a binary compatable for RPi3/4

```
./scripts/cross-build.sh
```
