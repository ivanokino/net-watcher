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

#[path = "./structs_impl.rs"]
mod structs_impl;
