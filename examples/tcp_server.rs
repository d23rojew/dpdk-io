use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

const HTML: &'static str = concat! {"HTTP/1.1 200 OK\r\n",
"Server: F-Stack\r\n",
"Date: Sat, 25 Feb 2017 09:26:33 GMT\r\n",
"Content-Type: text/html\r\n",
"Content-Length: 438\r\n",
"Last-Modified: Tue, 21 Feb 2017 09:44:03 GMT\r\n",
"Connection: keep-alive\r\n",
"Accept-Ranges: bytes\r\n",
"\r\n",
"<!DOCTYPE html>\r\n",
"<html>\r\n",
"<head>\r\n",
"<title>Welcome to F-Stack!</title>\r\n",
"<style>\r\n",
"    body {  \r\n",
"        width: 35em;\r\n",
"        margin: 0 auto; \r\n",
"        font-family: Tahoma, Verdana, Arial, sans-serif;\r\n",
"    }\r\n",
"</style>\r\n",
"</head>\r\n",
"<body>\r\n",
"<h1>Welcome to F-Stack!</h1>\r\n",
"\r\n",
"<p>For online documentation and support please refer to\r\n",
"<a href=\"http://F-Stack.org/\">F-Stack.org</a>.<br/>\r\n",
"\r\n",
"<p><em>Thank you for using F-Stack.</em></p>\r\n",
"</body>\r\n",
"</html>",
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 80);
    dpdk_io::service::bootstrap();
    // tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    let tcp_listenr = dpdk_io::dpdk_agent().listen(addr).expect("listen");

    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        let (mut stream, addr) = tcp_listenr.accept().await;
        log::info!("addr = {}", addr);
        let read_len = stream.read(&mut buf).await.expect("read");
        log::info!("read  = {}", String::from_utf8_lossy(&buf[0..read_len]));

        stream
            .write_all(HTML.as_bytes())
            .await
            .expect("need write success");
        log::info!("write success");
    }
}
