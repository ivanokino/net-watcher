use std::{
    borrow::Cow,
    net::{Ipv4Addr, Ipv6Addr},
};

#[derive(PartialEq, Clone, Copy)]
pub enum Tab {
    Socket,
    Connect,
}
#[repr(C)]
pub union HandlerOutput {
    pub sock_get: SockGet,
    pub conn_get: ConnGet,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IpAddr {
    pub ipv4: u32,
    pub ipv6: u128,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SockGet {
    pub out_type: u8,
    pub pid: u32,
    pub family: u32,
    pub s_type: u32,
    pub comm: [u8; 16],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ConnGet {
    pub out_type: u8,
    pub pid: u32,
    pub fd: u32,

    pub family: i16,
    pub addr: IpAddr,
}

impl ConnGet {
    #[allow(dead_code)]
    pub fn family_name(&self) -> Cow<'static, str> {
        match self.family {
            0 => Cow::Borrowed("AF_UNSPEC"),
            1 => Cow::Borrowed("AF_UNIX"),
            2 => Cow::Borrowed("AF_INET"),
            10 => Cow::Borrowed("AF_INET6"),
            16 => Cow::Borrowed("AF_NETLINK"),
            27 => Cow::Borrowed("AF_PACKET"),

            _ => Cow::Owned(self.family.to_string()),
        }
    }

    pub fn addr_string(&self) -> String {
        if self.family == 10 {
            unsafe {
                let address = Ipv6Addr::from(self.addr.ipv6.to_le_bytes());
                address.to_string()
            }
        } else {
            unsafe {
                let address = Ipv4Addr::from(self.addr.ipv4.to_le_bytes());
                address.to_string()
            }
        }
    }
}

impl SockGet {
    #[allow(dead_code)]
    pub fn comm_name(&self) -> &str {
        let len = self
            .comm
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(self.comm.len());
        std::str::from_utf8(&self.comm[..len]).unwrap_or("?")
    }

    #[allow(dead_code)]
    pub fn type_name(&self) -> Cow<'static, str> {
        let raw_type = self.s_type & 0xf;

        match raw_type {
            1 => Cow::Borrowed("SOCK_STREAM"),
            2 => Cow::Borrowed("SOCK_DGRAM"),
            3 => Cow::Borrowed("SOCK_RAW"),
            4 => Cow::Borrowed("SOCK_RDM"),
            5 => Cow::Borrowed("SOCK_SEQPACKET"),
            6 => Cow::Borrowed("SOCK_DCCP"),
            10 => Cow::Borrowed("SOCK_PACKET"),

            _ => Cow::Owned(self.s_type.to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn family_name(&self) -> Cow<'static, str> {
        match self.family {
            0 => Cow::Borrowed("AF_UNSPEC"),
            1 => Cow::Borrowed("AF_UNIX"),
            2 => Cow::Borrowed("AF_INET"),
            10 => Cow::Borrowed("AF_INET6"),
            16 => Cow::Borrowed("AF_NETLINK"),
            27 => Cow::Borrowed("AF_PACKET"),

            _ => Cow::Owned(self.family.to_string()),
        }
    }
}
