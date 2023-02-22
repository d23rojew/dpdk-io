use crate::dpdk_agent;
use crate::net::Socket;
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::os::fd::AsRawFd;
use std::os::fd::RawFd;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::{Context, Poll};
use tokio::io::ReadBuf;

pub struct TcpStream {
    fd: i32,

    inner: Arc<std::sync::RwLock<Socket>>,
}

pub struct TcpListener {
    fd: i32,
    accept_agent: Option<crate::service::AcceptAgent>,
    inner: Arc<std::sync::RwLock<Socket>>,
}

impl AsRawFd for TcpListener {
    fn as_raw_fd(&self) -> RawFd {
        return self.fd;
    }
}

impl AsRawFd for TcpStream {
    fn as_raw_fd(&self) -> RawFd {
        return self.fd;
    }
}

impl TcpListener {
    pub fn as_raw_fd(&self) -> RawFd {
        return self.fd;
    }

    pub(crate) fn from_socket(socket: Arc<std::sync::RwLock<Socket>>) -> Self {
        let socket_guard = socket.read().expect("read lock double");
        let fd = socket_guard.as_raw_fd();

        TcpListener {
            inner: socket.clone(),
            accept_agent: None,
            fd,
        }
    }

    pub(crate) fn set_accept_agent(&mut self, accpet_agent: crate::service::AcceptAgent) {
        self.accept_agent = Some(accpet_agent);
    }
}

impl TcpStream {
    pub fn new(fd: RawFd, inner: Arc<std::sync::RwLock<Socket>>) -> Self {
        Self { fd, inner }
    }
    pub fn as_raw_fd(&self) -> RawFd {
        return self.fd;
    }

    pub fn inner(&self) -> Arc<std::sync::RwLock<Socket>> {
        return self.inner.clone();
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        let addr: SocketAddr = unsafe { std::mem::zeroed() };
        return Ok(addr);
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        let addr: SocketAddr = unsafe { std::mem::zeroed() };
        return Ok(addr);
    }

    pub fn can_write(&self) -> bool {
        self.inner
            .read()
            .expect("TcpStream read inner can_write")
            .can_write()
    }

    pub fn can_read(&self) -> bool {
        self.inner
            .read()
            .expect("TcpStream read inner can_read")
            .can_read()
    }
}

impl tokio::io::AsyncRead for TcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        // log::trace!("tcp poll try read");
        if self.can_read() == false {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        // log::trace!("tcp poll read");

        let result = dpdk_agent().read(&self, buf.initialize_unfilled());
        match result {
            Ok(len) => {
                buf.advance(len);
                log::trace!("agent read = {}", String::from_utf8_lossy(buf.filled()));
                if len == 0 {
                    cx.waker().wake_by_ref();
                    return Poll::Pending;
                }
                return Poll::Ready(Ok(()));
            }
            Err(err) => {
                // log::trace!("agent read  err = {:?}", err.raw_os_error());
                if err.raw_os_error().unwrap() == 11 {
                    cx.waker().wake_by_ref();
                    return Poll::Pending;
                } else {
                    return Poll::Ready(Err(err));
                }
            }
        };
    }
}

impl tokio::io::AsyncWrite for TcpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        // log::trace!("tcp poll write");
        if self.can_write() == false {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }

        let result = dpdk_agent().write(&self, buf);
        match result {
            Ok(len) => Poll::Ready(Ok(len)),
            Err(err) => Poll::Ready(Err(err)),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub fn new_for_addr(address: SocketAddr) -> io::Result<libc::c_int> {
    let domain = match address {
        SocketAddr::V4(_) => libc::AF_INET,
        SocketAddr::V6(_) => libc::AF_INET6,
    };
    // println!("domain = {:?}", domain);
    let ret = unsafe { crate::fstack::ff_socket(domain, libc::SOCK_STREAM, libc::IPPROTO_TCP) };
    if ret < 0 {
        return Err(std::io::Error::last_os_error());
    }
    // println!("ret socket = {}", ret);
    return Ok(ret);
}

pin_project! {
    pub struct AcceptFuture{
        listner_fd:RawFd,
        accept_agent:Option<crate::service::AcceptAgent>,
        socket: Arc<std::sync::RwLock<Socket>>,
    }
}

impl Future for AcceptFuture {
    type Output = (TcpStream, SocketAddr);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // log::trace!("start to poll accept");
        if self.accept_agent.is_none() {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }

        // log::trace!("socket can read to poll accept");

        if let Some(agent) = &self.accept_agent {
            match agent.accpet(self.listner_fd) {
                Ok(Some((stream, addr))) => {
                    return Poll::Ready((stream, addr));
                }
                Ok(None) => {
                    cx.waker().wake_by_ref();
                    return Poll::Pending;
                }
                Err(_) => {
                    cx.waker().wake_by_ref();
                    return Poll::Pending;
                }
            };
        }
        cx.waker().wake_by_ref();
        return Poll::Pending;
    }
}

impl TcpListener {
    pub fn accept(&self) -> AcceptFuture {
        AcceptFuture {
            listner_fd: self.as_raw_fd(),
            accept_agent: self.accept_agent.clone(),
            socket: self.inner.clone(),
        }
    }
}
