use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use dpdk_io::dpdk_agent;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() {
    env_logger::init();
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(172, 31, 10, 131)), 80);
    dpdk_io::service::bootstrap();
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let mut r = dpdk_agent().connect(addr).expect("connect fail");

    log::info!("r. fd = {}", r.as_raw_fd());

    log::info!("connect cmd send success");
    let msg = b"GET /ping?ping_time=%ld HTTP/1.1\r\nHost: 172.31.10.131\r\nContent-Type: application/json\n\r\n";
    log::info!("time to process test");
    let n = r.write(msg).await.expect("need write success");
    // let n = rt.write(&r, msg).expect("write ");
    if n == 0 {
        panic!("write fail")
    }

    println!("write success = {}", n);

    loop {
        let mut buf: [u8; 10] = [0; 10];

        if 0 == r.read(&mut buf).await.expect("need read success") {
            break;
        }

        println!(
            "receive msg = {:?}",
            String::from_utf8_lossy(buf.as_slice())
        );
    }

    // tokio::io::AsyncRead::
}
