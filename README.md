# Shorten the parent directories names

Simple tool to shorten the directory names in the path

## Install using cargo

```bash
cargo install --git https://github.com/pestanko/rs-path-shortener
```

It will install the `shorten-path` to the `$HOME/.cargo/bin` (you need to add it to the `$PATH`)

## Build

To build and "install" the shortener to the `$HOME/.local/bin` please execute the install script.

```bash

./install.sh

```

## Configuration

Env variables to configure the shortener:

```bash
export SHORTHEN_DIR_PATH_LIMIT=1  # limit the length of the dir names
```
