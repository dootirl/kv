# kv
## Setup
This project is written in Rust, you'll need to have it installed on your system. You can download and install Rust from the official website: https://www.rust-lang.org/.

Additionally, you'll need `protoc` to compile the `.proto` files. You can install it by following the instructions here: https://grpc.io/docs/protoc-installation/.

## Build
To build it, enter those commands:
```
$ git clone https://github.com/dootirl/kv
$ cd kv
$ cargo build --release
```
After running them, you should have binaries `kv_store` and `kv_proxy` available under `./target/release`.

## Usage
### kv_store
`kv_store` is a key-value store reachable through gRPC and you can run it with:
```
$ ./target/release/kv_store
```
### kv_proxy
`kv_proxy` is a REST proxy to `kv_store`. Minimally it requires paths to HTTPS certificates which you can supply with `--cert-path` and `--key-path` flags. To generate those certificates, make sure you have OpenSSL installed and enter:
```
$ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256 -days 365 -nodes
```

Now to finally run it type:
```
$ ./target/release/kv_proxy
```

### 

### Log level
To see more data when the programs are running, you can adjust log level by preceding previous commands with `RUST_LOG=<log-level>` where `<log-level>` is one of the following values: `error`, `warn`, `info`, `debug`, `trace`.
