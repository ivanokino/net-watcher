use libbpf_rs::{MapHandle, RingBufferBuilder};
use std::net::TcpListener;
use std::{io::Write, mem, path::Path, time::Duration};

#[path = "../include/structs.rs"]
mod structs;
use crate::net::run_detail_streaming;
use crate::structs::SockGet;

#[path = "../include/net.rs"]
mod net;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8085")?;
    let (mut stream, _) = listener.accept()?;
    println!("back connected");

    let det_listener = TcpListener::bind("127.0.0.1:8086")?;
    let (det_stream, _) = det_listener.accept()?; //stream for detail information 

    run_detail_streaming(det_stream);

    let callback = move |data: &[u8]| -> i32 {
        if data.len() < mem::size_of::<SockGet>() {
            return 0;
        };
        // println!("got");
        let _ = stream.write_all(data);
        let _ = stream.flush();
        0
    };

    let path = Path::new("/sys/fs/bpf/my_prog/ringbuff");

    let map = MapHandle::from_pinned_path(path)?;
    let mut builder = RingBufferBuilder::new();
    builder.add(&map, callback)?;

    let ringbuf = builder.build()?;

    loop {
        let _ = ringbuf.poll(Duration::from_secs(5));
    }
}
