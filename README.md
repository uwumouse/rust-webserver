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

I assume you've already built it with `cargo build`:

```bash
./target/webserver ./config.yml
# Or you can try to run it with default config
./target/webserver ./config.example.yml
```
