# Rust WebServer

**Rust WebServer** is a simple static files http server. It allows you to configure it via `config.yml` file.

## Setup

You just need to download rustup. You can do it with this line:

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Configuration

Create a `config.yml` file.  
See a `config.example.yml` as a reference, it's pretty straight forward.

## Running

First of all, you need to build it:

```bash
cargo build --release
cp ./target/release/webserver .
```

And then run it (assuming you have a config file in current directory):

```bash
./webserver config.example.yml
```
