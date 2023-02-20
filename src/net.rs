use libc::c_void;
use std::ffi::CString;
use std::io;
use std::mem::size_of;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::os::fd::{AsRawFd, RawFd};

#[derive(Debug)]
pub struct Socket {
    fd: RawFd,
    can_read: bool,
    can_write: bool,
    lose_connect: bool,
}

impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        return self.fd;
    }
}

impl From<RawFd> for Socket {
    fn from(fd: RawFd) -> Self {
        Self {
            fd,
            can_read: false,
            can_write: false,
            lose_connect: false,
        }
    }
}

impl Socket {
    pub fn new(
        domain: libc::c_int,
        socket_type: libc::c_int,
        protocol: libc::c_int,
    ) -> io::Result<Socket> {
        let fd = unsafe { crate::fstack::ff_socket(domain, socket_type, protocol) };

        if fd < 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(Socket {
            fd,
            can_read: false,
            can_write: false,
            lose_connect: false,
        })
    }

    #[inline]
    pub fn can_read(&self) -> bool {
        return self.can_read;
    }

    #[inline]
    pub fn can_write(&self) -> bool {
        return self.can_write;
    }

    #[inline]
    pub fn lose_connect(&self) -> bool {
        return self.lose_connect;
    }

    #[inline]
    pub fn set_can_read(&mut self, v: bool) {
        self.can_read = v;
    }

    #[inline]
    pub fn set_can_write(&mut self, v: bool) {
        self.can_write = v;
    }

    #[inline]
    pub fn set_lose_connect(&mut self, v: bool) {
        self.lose_connect = v;
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let ret = unsafe {
            crate::fstack::ff_read(self.fd, buf.as_mut_ptr() as *mut c_void, buf.len() as u64)
        };
        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(ret as usize)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let ret = unsafe {
            crate::fstack::ff_write(self.fd, buf.as_ptr() as *const c_void, buf.len() as u64)
        };

        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(ret as usize)
    }

    pub fn as_raw_fd(&self) -> RawFd {
        return self.fd;
    }
}

#[repr(C)]
pub(crate) union SocketAddrCRepr {
    v4: crate::fstack::sockaddr_in,
    v6: crate::fstack::sockaddr_in6,
}

impl SocketAddrCRepr {
    pub(crate) fn as_ptr(&self) -> *const libc::sockaddr {
        self as *const _ as *const libc::sockaddr
    }
}

pub(crate) fn socket_addr(addr: &SocketAddr) -> (SocketAddrCRepr, libc::socklen_t) {
    match addr {
        SocketAddr::V4(ref addr) => {
            // `s_addr` is stored as BE on all machine and the array is in BE order.
            // So the native endian conversion method is used so that it's never swapped.
            let mut sin_addr: crate::fstack::in_addr = unsafe { std::mem::zeroed() };

            let addr_string = CString::new(addr.ip().to_string()).unwrap();
            let pton_ret = unsafe {
                crate::fstack::inet_pton(
                    libc::AF_INET as i32,
                    addr_string.as_ptr(),
                    &mut sin_addr as *mut crate::fstack::in_addr as *mut c_void,
                )
            };

            if pton_ret != 1 {
                panic!("\nInvalid address/ Address not supported \n");
            }

            let sockaddr_in = crate::fstack::sockaddr_in {
                sin_family: libc::AF_INET as libc::sa_family_t,
                sin_port: unsafe { crate::fstack::htons(80u16) },
                sin_addr,
                sin_zero: [0; 8],
            };

            println!("servaddr = {:?}", sockaddr_in);

            let sockaddr = SocketAddrCRepr { v4: sockaddr_in };
            let socklen = size_of::<libc::sockaddr_in>() as libc::socklen_t;
            (sockaddr, socklen)
        }
        SocketAddr::V6(ref addr) => {
            let sockaddr_in6 = crate::fstack::sockaddr_in6 {
                sin6_family: libc::AF_INET6 as libc::sa_family_t,
                sin6_port: addr.port().to_be(),
                sin6_addr: crate::fstack::in6_addr {
                    __in6_u: crate::fstack::in6_addr__bindgen_ty_1 {
                        __u6_addr8: addr.ip().octets(),
                    },
                },
                sin6_flowinfo: addr.flowinfo(),
                sin6_scope_id: addr.scope_id(),
            };

            let sockaddr = SocketAddrCRepr { v6: sockaddr_in6 };
            let socklen = size_of::<libc::sockaddr_in6>() as libc::socklen_t;
            (sockaddr, socklen)
        }
    }
}

pub(crate) unsafe fn to_socket_addr(
    storage: *const libc::sockaddr_storage,
) -> io::Result<SocketAddr> {
    match (*storage).ss_family as libc::c_int {
        libc::AF_INET => {
            // Safety: if the ss_family field is AF_INET then storage must be a sockaddr_in.
            let addr: &libc::sockaddr_in = &*(storage as *const libc::sockaddr_in);
            let ip = Ipv4Addr::from(addr.sin_addr.s_addr.to_ne_bytes());
            let port = u16::from_be(addr.sin_port);
            Ok(SocketAddr::V4(SocketAddrV4::new(ip, port)))
        }
        libc::AF_INET6 => {
            // Safety: if the ss_family field is AF_INET6 then storage must be a sockaddr_in6.
            let addr: &libc::sockaddr_in6 = &*(storage as *const libc::sockaddr_in6);
            let ip = Ipv6Addr::from(addr.sin6_addr.s6_addr);
            let port = u16::from_be(addr.sin6_port);
            Ok(SocketAddr::V6(SocketAddrV6::new(
                ip,
                port,
                addr.sin6_flowinfo,
                addr.sin6_scope_id,
            )))
        }
        _ => Err(io::ErrorKind::InvalidInput.into()),
    }
}
