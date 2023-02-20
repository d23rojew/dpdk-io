# dpdk-io
It is a DPDK TcpStream/UDP-Socket realized by RUST

## prepare

### dpdk

### fstack



## example

```
    cargo build
    RUST_LOG=TRACE ./target/debug/tcp_server --conf config.ini  --proc-type=primary --proc-id=0
    RUST_LOG=TRACE ./target/debug/tcp_client --conf config.ini  --proc-type=primary --proc-id=0

```