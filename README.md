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

## 试验

ave cost = 287.392µs
ave cost = 285.79µs
ave cost = 280.058µs
ave cost = 278.775µs
ave cost = 280.702µs


ave cost = 131.514µs
ave cost = 135.823µs
ave cost = 138.383µs
ave cost = 138.952µs
ave cost = 138.682µs

