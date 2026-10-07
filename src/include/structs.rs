use std::borrow::Cow;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SockGet {
    pub pid: u32,
    pub family: u32,
    pub s_type: u32,
    pub comm: [u8; 16],
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
