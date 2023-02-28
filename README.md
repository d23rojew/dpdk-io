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


## std tcp 2

ave cost = 227.861µs
ave cost = 233.596µs
ave cost = 219.172µs
ave cost = 260.543µs
ave cost = 262.681µs

239.8



## dpdk block
ave cost = 231.619µs
ave cost = 262.893µs
ave cost = 257.692µs
ave cost = 265.751µs
ave cost = 243.777µs

251.6


64 bytes from 172.31.10.131: icmp_seq=1 ttl=64 time=0.276 ms
64 bytes from 172.31.10.131: icmp_seq=2 ttl=64 time=0.229 ms
64 bytes from 172.31.10.131: icmp_seq=3 ttl=64 time=0.223 ms
64 bytes from 172.31.10.131: icmp_seq=4 ttl=64 time=0.220 ms
64 bytes from 172.31.10.131: icmp_seq=5 ttl=64 time=0.225 ms

## std tcp 3
ave cost = 256.69µs
ave cost = 245.743µs
ave cost = 242.844µs
ave cost = 222.867µs
ave cost = 266.221µs

246.2

## dpdk block
ave cost = 251.772µs
ave cost = 267.354µs
ave cost = 250.964µs
ave cost = 242.907µs
ave cost = 240.792µs

250.0

## tokio
ave cost = 241.096µs
ave cost = 243.419µs
ave cost = 266.085µs
ave cost = 251.893µs
ave cost = 255.302µs

251.2

## dpdk-io
ave cost = 252.182µs
ave cost = 249.331µs
ave cost = 241.303µs
ave cost = 243.036µs
ave cost = 242.929µs

245.4

## ping
64 bytes from 172.31.10.131: icmp_seq=1 ttl=64 time=0.267 ms
64 bytes from 172.31.10.131: icmp_seq=2 ttl=64 time=0.218 ms
64 bytes from 172.31.10.131: icmp_seq=3 ttl=64 time=0.220 ms
64 bytes from 172.31.10.131: icmp_seq=4 ttl=64 time=0.254 ms
64 bytes from 172.31.10.131: icmp_seq=5 ttl=64 time=0.217 ms
## STD 裸 tcp
ave cost = 261.969µs
ave cost = 239.588µs
ave cost = 248.675µs
ave cost = 228.368µs
ave cost = 246.255µs
2500次请求平均耗时 244.4us
## DPDK 裸 tcp
ave cost = 262.691µs
ave cost = 258.701µs
ave cost = 238.435µs
ave cost = 257.564µs
ave cost = 241.495µs
2500次请求平均耗时 251.2us
## Tokio tcp
ave cost = 243.624µs
ave cost = 266.086µs
ave cost = 285.713µs
ave cost = 275.582µs
ave cost = 290.334µs
2500次请求平均耗时 271.8
## DPDK with Tokio tcp
ave cost = 242.248µs
ave cost = 245.022µs
ave cost = 261.979µs
ave cost = 268.91µs
ave cost = 270.546µs
2500次请求平均耗时 257.2


## STD 裸 tcp
ave cost = 237.528µs
ave cost = 245.779µs
ave cost = 239.674µs
ave cost = 229.782µs
ave cost = 250.41µs

## DPDK 裸 tcp
ave cost = 252.056µs
ave cost = 264.112µs
ave cost = 261.593µs
ave cost = 268.676µs
ave cost = 227.751µs
## Tokio tcp
ave cost = 247.22µs
ave cost = 279.09µs
ave cost = 252.893µs
ave cost = 241.264µs
ave cost = 248.787µs
## DPDK with Tokio tcp
ave cost = 259.97µs
ave cost = 257.619µs
ave cost = 244.577µs
ave cost = 242.835µs
ave cost = 259.783µs


ave cost = 245.928µs
ave cost = 262.766µs
ave cost = 236.853µs
ave cost = 243.094µs
ave cost = 257.429µs
248.6us

ave cost = 242.39µs
ave cost = 251.592µs
ave cost = 269.688µs
ave cost = 244.616µs
ave cost = 241.058µs

249.4

ave cost = 263.216µs
ave cost = 275.01µs
ave cost = 265.126µs
ave cost = 244.612µs
ave cost = 250µs

259.4

ave cost = 243.029µs
ave cost = 238.646µs
ave cost = 246.488µs
ave cost = 269.829µs
ave cost = 235.786µs

246.2

224.335µs
243.316µs
256.759µs
286.818µs