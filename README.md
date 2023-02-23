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

## 第一次试验(100ms 请求一次,请求100次)
ave cost = 287.392µs
ave cost = 285.79µs
ave cost = 280.058µs
ave cost = 278.775µs
ave cost = 280.702µs

## 第二次试验(连续请求100次)
ave cost = 131.514µs
ave cost = 135.823µs
ave cost = 138.383µs
ave cost = 138.952µs
ave cost = 138.682µs

## 第三次试验(设置keepAlive连续请求100次)
ave cost = 140.058µs
ave cost = 149.392µs
ave cost = 141.812µs
ave cost = 143.184µs
ave cost = 143.001µs

## 第四次试验(重复第二次试验)
ave cost = 145.269µs
ave cost = 145.603µs
ave cost = 138.131µs
ave cost = 148.022µs
ave cost = 141.236µs



## 第五次试验 tokio_tcp
ave cost = 144.082µs
ave cost = 147.273µs
ave cost = 150.463µs
ave cost = 143.764µs
ave cost = 143.469µs

开启no_delay
ave cost = 142.559µs
ave cost = 145.94µs
ave cost = 146.914µs
ave cost = 145.264µs
ave cost = 144.67µs

## 第六次 dpdk-tcp
ave cost = 136.745µs
ave cost = 136.76µs
ave cost = 137.072µs
ave cost = 130.012µs
ave cost = 138.625µs

## std tcp
ave cost = 95.977µs
ave cost = 96.15µs
ave cost = 102.911µs
ave cost = 98.489µs
ave cost = 92.649µs