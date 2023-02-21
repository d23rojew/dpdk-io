use crate::fstack::epoll_data;
use crate::fstack::epoll_event;
use crate::tcp::TcpListener;
use crate::tcp::TcpStream;
use libc::c_void;
use libc::EPOLLET;
use libc::EPOLLHUP;
use libc::EPOLLIN;
use libc::EPOLLOUT;
use libc::EPOLL_CTL_ADD;
use std::ffi::{c_char, CString};
use std::net::SocketAddr;
use std::os::fd::RawFd;
use std::sync::Arc;
use std::sync::Mutex;

static mut AGENT: Option<Arc<Agent>> = None;
static mut DPDK_ARG: Vec<String> = vec![];

pub fn set_dpdk_arg(arg: Vec<String>) {
    unsafe {
        for e in arg.into_iter() {
            DPDK_ARG.push(e);
        }
    }
}

pub fn dpdk_agent() -> Arc<Agent> {
    unsafe {
        if AGENT.is_none() {
            panic!("agent not init ")
        }
    }
    unsafe { AGENT.as_ref().unwrap().clone() }
}

pub struct RawPointer(*const u8);
pub struct RawMutPointer(*mut u8);
pub struct WriteArguementTuple(
    i32,
    RawPointer,
    usize,
    oneshot::Sender<Result<usize, std::io::Error>>,
);

impl RawPointer {
    fn as_ptr(&self) -> *const u8 {
        return self.0;
    }
}

impl RawMutPointer {
    fn as_mut_ptr(&self) -> *mut u8 {
        return self.0;
    }
}

pub struct ReadArguementTuple(
    i32,
    RawMutPointer,
    usize,
    oneshot::Sender<Result<usize, std::io::Error>>,
);

unsafe impl Send for WriteArguementTuple {}
unsafe impl Sync for WriteArguementTuple {}

unsafe impl Send for ReadArguementTuple {}
unsafe impl Sync for ReadArguementTuple {}

const MAX_EVENTS: i32 = 1024;

pub struct Agent {
    listen_sender: std::sync::mpsc::Sender<(
        SocketAddr,
        oneshot::Sender<Result<TcpListener, std::io::Error>>,
    )>,
    connect_sender: std::sync::mpsc::Sender<(
        SocketAddr,
        oneshot::Sender<Result<TcpStream, std::io::Error>>,
    )>,
    read_agent: ReadAgent,
    write_agent: WriteAgent,
    accept_agent: AcceptAgent,
}

#[derive(Clone)]
struct ReadSender(std::sync::mpsc::Sender<ReadArguementTuple>);
unsafe impl Send for ReadSender {}
unsafe impl Sync for ReadSender {}

#[derive(Clone)]
struct WriteSender(std::sync::mpsc::Sender<WriteArguementTuple>);
unsafe impl Send for WriteSender {}
unsafe impl Sync for WriteSender {}

#[derive(Clone)]
pub struct ReadAgent {
    read_sender: ReadSender,
}

impl ReadAgent {
    pub fn read(&self, tcp_stream: &TcpStream, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let (result_sender, result_receive) = oneshot::channel();
        let arg = ReadArguementTuple(
            tcp_stream.as_raw_fd(),
            RawMutPointer(buf.as_mut_ptr()),
            buf.len(),
            result_sender,
        );
        log::trace!("start to read tcp:{},", tcp_stream.as_raw_fd());
        self.read_sender.0.send(arg).expect("send read cmd fail");
        return result_receive.recv().expect("receive read result");
    }
}

#[derive(Clone)]
pub struct WriteAgent {
    write_sender: WriteSender,
}

impl WriteAgent {
    pub fn write(&self, tcp_stream: &TcpStream, data: &[u8]) -> Result<usize, std::io::Error> {
        log::trace!(
            "send write fd:{} cmd:data = {}",
            tcp_stream.as_raw_fd(),
            String::from_utf8_lossy(data)
        );
        let (result_sender, result_receive) = oneshot::channel();
        let arg = WriteArguementTuple(
            tcp_stream.as_raw_fd(),
            RawPointer(data.as_ptr()),
            data.len(),
            result_sender,
        );
        self.write_sender.0.send(arg).expect("send write cmd fail");
        log::trace!("send write cmd success");
        return result_receive.recv().expect("receive write result");
    }
}

#[derive(Clone)]
pub struct AcceptAgent {
    accept_sender:
        std::sync::mpsc::Sender<(RawFd, oneshot::Sender<Option<(TcpStream, SocketAddr)>>)>,
    read_agent: ReadAgent,
    write_agent: WriteAgent,
}

impl AcceptAgent {
    pub fn accpet(&self, fd: RawFd) -> Result<Option<(TcpStream, SocketAddr)>, std::io::Error> {
        // log::trace!("start to send accpet cmd");
        let (result_sender, result_receive) = oneshot::channel();
        self.accept_sender
            .send((fd, result_sender))
            .expect("send accpet cmd");
        // log::trace!("send accept cmd done");
        let r = result_receive.recv().expect("receive write result");
        if let Some((mut tcp_stream, addr)) = r {
            log::trace!("receive accept return {:?}", addr);
            tcp_stream.set_write_agent(Mutex::new(self.write_agent.clone()));
            tcp_stream.set_read_agent(Mutex::new(self.read_agent.clone()));
            log::trace!("end accpet cmd get tcp :{}", addr);
            return Ok(Some((tcp_stream, addr)));
        }
        // log::trace!("receive accept None");
        Ok(None)
    }
}

impl Agent {
    pub fn connect_to(&self, addr: SocketAddr) -> Result<TcpStream, std::io::Error> {
        let (result_sender, result_receive) = oneshot::channel();
        self.connect_sender
            .send((addr, result_sender))
            .expect("send connect cmd fail");

        if let Ok(mut tcp_stream) = result_receive.recv().expect("receive connect result") {
            log::trace!("start to wait connect success ");
            tcp_stream.wait_connect_success().expect("wait success");

            tcp_stream.set_read_agent(Mutex::new(self.read_agent.clone()));
            tcp_stream.set_write_agent(Mutex::new(self.write_agent.clone()));
            return Ok(tcp_stream);
        }

        return Err(std::io::Error::last_os_error());
    }

    pub fn connect_to_timeout(
        &self,
        addr: SocketAddr,
        timeout: std::time::Duration,
    ) -> Result<TcpStream, std::io::Error> {
        let (result_sender, result_receive) = oneshot::channel();
        self.connect_sender
            .send((addr, result_sender))
            .expect("send connect cmd fail");

        if let Ok(mut tcp_stream) = result_receive.recv().expect("recv connect result")
        // .expect("receive connect result")
        {
            log::trace!("start to wait connect success ");
            if let Err(crate::error::Error::Timeout(err)) =
                tcp_stream.wait_connect_success_timeout(timeout)
            {
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, err));
            }

            tcp_stream.set_read_agent(Mutex::new(self.read_agent.clone()));
            tcp_stream.set_write_agent(Mutex::new(self.write_agent.clone()));
            return Ok(tcp_stream);
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "connection timed out",
            ));
        }
    }

    pub fn connect(&self, addr: SocketAddr) -> Result<TcpStream, std::io::Error> {
        for _ in 0..3 {
            if let Ok(r) = self.connect_to_timeout(addr, std::time::Duration::from_secs(1)) {
                return Ok(r);
            }
            log::trace!("connect timeout retry");
        }
        return Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "connection timed out",
        ));
    }

    pub fn listen(&self, addr: SocketAddr) -> Result<TcpListener, std::io::Error> {
        log::trace!("start to send listen cmd");
        println!("start to send listen cmd");
        let (result_sender, result_receive) = oneshot::channel();

        self.listen_sender
            .send((addr, result_sender))
            .expect("send listen cmd fail");

        if let Ok(mut tcp_listener) = result_receive.recv().expect("receive connect result") {
            tcp_listener.set_accept_agent(self.accept_agent.clone());
            log::trace!("get accept agent");
            return Ok(tcp_listener);
        }

        return Err(std::io::Error::last_os_error());
    }

    pub fn write(&self, tcp_stream: &TcpStream, data: &[u8]) -> Result<usize, std::io::Error> {
        self.write_agent.write(tcp_stream, data)
    }

    pub fn read(&self, tcp_stream: &TcpStream, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.read_agent.read(tcp_stream, buf)
    }

    pub fn accpet(&self, fd: RawFd) -> Result<Option<(TcpStream, SocketAddr)>, std::io::Error> {
        self.accept_agent.accpet(fd)
    }
}

#[repr(C)]
struct Registry {
    epoll_fd: RawFd,
    listen: std::sync::mpsc::Receiver<(
        SocketAddr,
        oneshot::Sender<Result<TcpListener, std::io::Error>>,
    )>,
    connect: std::sync::mpsc::Receiver<(
        SocketAddr,
        oneshot::Sender<Result<TcpStream, std::io::Error>>,
    )>,
    connect_wait: std::collections::BTreeMap<RawFd, std::sync::mpsc::Sender<bool>>,
    read: std::sync::mpsc::Receiver<ReadArguementTuple>,
    write: std::sync::mpsc::Receiver<WriteArguementTuple>,
    conn: std::collections::BTreeMap<RawFd, Arc<std::sync::RwLock<crate::net::Socket>>>,
    listener: std::collections::BTreeMap<RawFd, Arc<std::sync::RwLock<crate::net::Socket>>>,
    can_accept: std::collections::BTreeMap<RawFd, bool>,
    accept: std::sync::mpsc::Receiver<(RawFd, oneshot::Sender<Option<(TcpStream, SocketAddr)>>)>,
    conn_belong_listener: std::collections::BTreeMap<RawFd, RawFd>,
}

impl Registry {
    unsafe fn process_connect(&mut self) {
        loop {
            // log::trace!("loop connect");
            match self.connect.try_recv() {
                Ok((addr, sender)) => {
                    println!("receive connect cmd  addr = {:?}", addr);
                    let socket = crate::net::Socket::new(
                        libc::AF_INET,
                        libc::SOCK_STREAM,
                        libc::IPPROTO_TCP,
                    )
                    .expect("new tcp socket");

                    let fd = socket.as_raw_fd();

                    let on = 1;
                    unsafe { crate::fstack::ff_ioctl(fd, libc::FIONBIO, &on) };

                    let mut ev = epoll_event {
                        events: EPOLLOUT as u32 | EPOLLIN as u32 | EPOLLET as u32,
                        data: epoll_data { fd },
                    };

                    crate::fstack::ff_epoll_ctl(
                        self.epoll_fd,
                        EPOLL_CTL_ADD as i32,
                        fd,
                        &mut ev as *mut epoll_event,
                    );

                    let (raw_addr, raw_addr_length) = crate::net::socket_addr(&addr);

                    let ret = unsafe {
                        crate::fstack::ff_connect(
                            fd,
                            raw_addr.as_ptr() as *const libc::sockaddr
                                as *const crate::fstack::linux_sockaddr,
                            raw_addr_length,
                        )
                    };

                    if ret < 0 && std::io::Error::last_os_error().raw_os_error().unwrap() != 115 {
                        sender
                            .send(Err(std::io::Error::last_os_error()))
                            .expect("send connect result fail");
                        continue;
                    }

                    log::trace!(
                        "connect ret = {},err = {:?}",
                        ret,
                        std::io::Error::last_os_error()
                    );

                    let socket = Arc::new(std::sync::RwLock::new(socket));

                    self.conn.insert(fd, socket.clone());

                    let (connect_sender, receiver) = std::sync::mpsc::channel();

                    let stream = TcpStream::new(fd, Mutex::new(receiver), socket);

                    self.connect_wait.insert(fd, connect_sender);

                    sender.send(Ok(stream)).expect("send connect result fail");

                    log::trace!("async connect done");
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    unsafe fn process_accept(&mut self) {
        loop {
            // log::trace!("start to receive accpet cmd ");
            if let Ok((listen_fd, c)) = self.accept.try_recv() {
                // log::trace!("recevie cmd of accept :{}", listen_fd);
                if self.can_accept.get(&listen_fd).is_none()
                    || !self.can_accept.get(&listen_fd).unwrap()
                {
                    c.send(None).expect("send accpet client fail");
                    break;
                }
                self.can_accept.remove(&listen_fd);
                let mut addr: crate::fstack::linux_sockaddr = std::mem::zeroed();
                let mut addr_len: i32 = 0;
                let client_fd = crate::fstack::ff_accept(
                    listen_fd,
                    &mut addr as *mut crate::fstack::linux_sockaddr,
                    &mut addr_len as *mut i32 as *mut std::os::raw::c_void
                        as *mut crate::fstack::socklen_t,
                );

                if client_fd < 0 {
                    log::trace!("ff_accept new stream fail:{}", client_fd);
                    c.send(None).expect("send accpet client fail");
                    break;
                }

                let mut new_ev: epoll_event = unsafe { std::mem::zeroed() };
                new_ev.data.fd = client_fd;
                new_ev.events = EPOLLIN as u32 | EPOLLET as u32;

                if crate::fstack::ff_epoll_ctl(
                    self.epoll_fd,
                    EPOLL_CTL_ADD,
                    client_fd,
                    &mut new_ev as *mut epoll_event,
                ) != 0
                {
                    log::trace!("ff_epoll_ctl new stream fail:{}", client_fd);
                    c.send(None).expect("send accpet epoll result");
                    break;
                }

                let socket = Arc::new(std::sync::RwLock::new(crate::net::Socket::from(client_fd)));
                self.conn.insert(client_fd, socket.clone());
                self.conn_belong_listener.insert(client_fd, listen_fd);
                let (connect_sender, receiver) = std::sync::mpsc::channel();
                let stream = TcpStream::new(client_fd, Mutex::new(receiver), socket);
                connect_sender.send(true).expect("send conn result");
                c.send(Some((
                    stream,
                    crate::net::to_socket_addr(
                        &addr as *const crate::fstack::linux_sockaddr as *const libc::c_void
                            as *const libc::sockaddr_storage,
                    )
                    .expect("trans linux add to rust socketAddr"),
                )))
                .expect("send accpet result");

                log::trace!("accpet loop one new stream");
            } else {
                break;
            }
        }
    }

    unsafe fn prorcess_listen(&mut self) {
        if let Ok((addr, c)) = self.listen.try_recv() {
            // log::trace!("dpdk process listen");
            let socket =
                crate::net::Socket::new(libc::AF_INET, libc::SOCK_STREAM, libc::IPPROTO_TCP)
                    .expect("");

            let socket_fd = socket.as_raw_fd();

            // let on = 1;
            // unsafe { crate::fstack::ff_ioctl(socket_fd, libc::FIONBIO, &on) };

            let (raw_addr, raw_addr_length) = crate::net::socket_addr(&addr);

            let ret = unsafe {
                crate::fstack::ff_bind(
                    socket_fd,
                    raw_addr.as_ptr() as *const libc::sockaddr
                        as *const crate::fstack::linux_sockaddr,
                    raw_addr_length,
                )
            };

            if ret < 0 {
                log::trace!("bind fail");
                c.send(Err(std::io::Error::last_os_error()))
                    .expect("send ff_bind error");
                return;
            }

            log::trace!("bind :{} done", socket_fd);

            let mut ev = epoll_event {
                events: EPOLLOUT as u32 | EPOLLET as u32 | EPOLLIN as u32,
                data: epoll_data { fd: socket_fd },
            };

            let ret = crate::fstack::ff_epoll_ctl(
                self.epoll_fd,
                EPOLL_CTL_ADD as i32,
                socket_fd,
                &mut ev as *mut epoll_event,
            );

            if ret < 0 {
                log::trace!("ff_epoll_ctl fail");
                c.send(Err(std::io::Error::last_os_error()))
                    .expect("send ff_epoll_ctl error");
                return;
            }

            let ret = unsafe { crate::fstack::ff_listen(socket_fd, MAX_EVENTS) };
            if ret < 0 {
                log::trace!("ff_listen fail");
                c.send(Err(std::io::Error::last_os_error()))
                    .expect("send ff_listen error");
                return;
            }

            let socket = Arc::new(std::sync::RwLock::new(socket));

            let listener = crate::tcp::TcpListener::from_socket(socket.clone());

            self.listener.insert(socket_fd, socket);

            c.send(Ok(listener)).expect("send listen");
        }
    }

    unsafe fn process_read(&mut self) {
        loop {
            // log::trace!("loop read");
            match self.read.try_recv() {
                Ok(arg) => {
                    log::trace!("start to read {},nbytes = {}", arg.0, arg.2);
                    let read_len = crate::fstack::ff_recv(
                        arg.0,
                        arg.1.as_mut_ptr() as *mut libc::c_void,
                        arg.2 as u64,
                        libc::MSG_WAITALL,
                    );
                    log::trace!("read fd:{} len is {}", arg.0, read_len);
                    if read_len == -1 {
                        log::trace!("read return -1 {:?}", std::io::Error::last_os_error());
                        arg.3
                            .send(Ok(0 as usize))
                            .expect("send process read result");
                    } else {
                        if read_len.is_negative() {
                            arg.3
                                .send(Err(std::io::Error::last_os_error()))
                                .expect("send process read err");
                        } else {
                            arg.3
                                .send(Ok(read_len as usize))
                                .expect("send process read result")
                        }
                    }
                }
                Err(_) => break,
            }
        }
    }

    unsafe fn process_write(&self) {
        loop {
            // log::trace!("loop read");
            match self.write.try_recv() {
                Ok(arg) => {
                    log::trace!("start to write");
                    let send_len = crate::fstack::ff_send(
                        arg.0,
                        arg.1.as_ptr() as *const libc::c_void,
                        arg.2 as u64,
                        libc::MSG_WAITALL,
                    );
                    log::trace!("send message done {}", send_len);
                    if send_len.is_negative() {
                        arg.3
                            .send(Err(std::io::Error::last_os_error()))
                            .expect("send process write err");
                    } else {
                        arg.3
                            .send(Ok(send_len as usize))
                            .expect("send process send_len result")
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
}

unsafe extern "C" fn dpdk_loop(arg: *mut c_void) -> i32 {
    let arg = arg as *mut Registry;
    let mut events = Vec::with_capacity(MAX_EVENTS as usize);
    // log::trace!("start to loop fd:{}", (*arg).epoll_fd);
    let nevents = crate::fstack::ff_epoll_wait((*arg).epoll_fd, events.as_mut_ptr(), MAX_EVENTS, 1);
    if nevents == 0 {
        (*arg).process_connect();
        (*arg).prorcess_listen();
        (*arg).process_accept();
        (*arg).process_read();
        (*arg).process_write();
        return 0;
    } else {
        if nevents == -1 {
            panic!("ff_epoll_wait receive -1")
        }

        log::trace!("get nevents = {}", nevents);

        unsafe { events.set_len(nevents as usize) };
        for i in 0..nevents {
            let fd = events[i as usize].data.fd as i32;
            log::trace!(
                "epoll wait get fd:{} event:{}",
                fd,
                events[i as usize].events as u32
            );

            if events[i as usize].events & EPOLLIN as u32 > 0 {
                //  client
                log::trace!("epoll wait get can read:{}", fd);
                if let Some(conn) = (*arg).conn.get(&fd) {
                    conn.write()
                        .expect("process in event conn read lock double")
                        .set_can_read(true);
                    log::trace!("set can read = true :{}", fd);
                } else if (*arg).listener.contains_key(&fd) {
                    (*arg).can_accept.insert(fd, true);
                    log::trace!("set can read = true :{}", fd);
                }
            } else if events[i as usize].events & EPOLLOUT as u32 > 0 {
                log::trace!("epoll wait get can write:{}", fd);
                if let Some(conn) = (*arg).conn.get(&fd) {
                    log::trace!("can write include conn");
                    conn.write()
                        .expect("process out event conn read lock double")
                        .set_can_write(true);
                    if let Some(sender) = (*arg).connect_wait.get(&fd) {
                        sender.send(true).expect("send conn result");
                        (*arg).connect_wait.remove(&fd);
                    }
                } else {
                    log::trace!("can write not include conn");
                }
            } else if events[i as usize].events & EPOLLHUP as u32 > 0 {
                log::trace!("epoll wait get hup :{}", fd);

                (*arg).conn.remove(&fd);
                if let Some(sender) = (*arg).connect_wait.get(&fd) {
                    sender.send(false).expect("send conn result");
                    (*arg).connect_wait.remove(&fd);
                }
            }
        }
    }

    return 0;
}

pub fn bootstrap() {
    let (listen_sender, listen_receiver) = std::sync::mpsc::channel();
    let (connect_sender, connect_receiver) = std::sync::mpsc::channel();
    let (read_sender, read_receiver) = std::sync::mpsc::channel();
    let (write_sender, write_receiver) = std::sync::mpsc::channel();
    let (accept_sender, accept_receiver) = std::sync::mpsc::channel();
    let agent = Agent {
        listen_sender,
        connect_sender,
        read_agent: ReadAgent {
            read_sender: ReadSender(read_sender.clone()),
        },
        write_agent: WriteAgent {
            write_sender: WriteSender(write_sender.clone()),
        },
        accept_agent: AcceptAgent {
            accept_sender,
            write_agent: WriteAgent {
                write_sender: WriteSender(write_sender),
            },
            read_agent: ReadAgent {
                read_sender: ReadSender(read_sender),
            },
        },
    };

    unsafe { AGENT = Some(Arc::new(agent)) }

    let (wait_prepare_done_sender, wait_prepare_done_receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let argv: Vec<String>;
        if unsafe { DPDK_ARG.len() } == 0 {
            argv = vec![
                String::from("./app"),
                String::from("--conf"),
                String::from("config.ini"),
                String::from("--proc-type=primary"),
                String::from("--proc-id=0"),
            ];
        } else {
            unsafe {
                argv = DPDK_ARG.iter().map(|e| e.clone()).collect();
            }
        }

        let cstr_argv: Vec<_> = argv
            .iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();

        let p_argv: Vec<_> = cstr_argv
            .iter()
            .map(|arg| arg.as_ptr() as *mut c_char)
            .collect();

        let p: *const *mut c_char = p_argv.as_ptr();

        let ret = unsafe { crate::fstack::ff_init(argv.len() as i32, p) };
        if ret < 0 {
            panic!("ff init ret = {}", ret);
        }

        let epoll_fd = unsafe { crate::fstack::ff_epoll_create(MAX_EVENTS) };

        let mut Selector = Registry {
            can_accept: std::collections::BTreeMap::new(),
            accept: accept_receiver,
            connect: connect_receiver,
            connect_wait: std::collections::BTreeMap::new(),
            epoll_fd,
            conn: std::collections::BTreeMap::new(),
            listener: std::collections::BTreeMap::new(),
            read: read_receiver,
            write: write_receiver,
            listen: listen_receiver,
            conn_belong_listener: std::collections::BTreeMap::new(),
        };

        wait_prepare_done_sender
            .send(())
            .expect("send prepare done");

        unsafe {
            crate::fstack::ff_run(
                Some(dpdk_loop),
                &mut Selector as *mut Registry as *mut std::os::raw::c_void,
            )
        };
    });

    let _ = wait_prepare_done_receiver
        .recv()
        .expect("wait dpdk prepare done");
}
