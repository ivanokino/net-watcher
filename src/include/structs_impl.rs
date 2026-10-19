use crate::structs::{ConnGet, SockGet};
use std::borrow::Cow;
use std::net::{Ipv4Addr, Ipv6Addr};
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
        match self.family {
            10 => unsafe {
                let address = Ipv6Addr::from(self.addr.ipv6.to_le_bytes());
                address.to_string()
            },

            2 => unsafe {
                let address = Ipv4Addr::from(self.addr.ipv4.to_le_bytes());
                address.to_string()
            },

            _ => "-".to_string(),
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
