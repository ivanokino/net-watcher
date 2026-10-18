use std::{path::Path, thread::JoinHandle, time::Duration};

use libbpf_rs::{MapHandle, RingBufferBuilder};

pub fn handle_socket(
    callback: impl FnMut(&[u8]) -> i32 + 'static,
) -> Result<JoinHandle<()>, Box<dyn std::error::Error>> {
    let path_socket = Path::new("/sys/fs/bpf/my_prog/ringbuff");

    let map = MapHandle::from_pinned_path(path_socket)?;
    let mut builder = RingBufferBuilder::new();
    builder.add(&map, callback)?;

    let ringbuf = builder.build()?;

    let thread = std::thread::spawn(move || {
        loop {
            let _ = ringbuf.poll(Duration::from_secs(5));
        }
    });

    Ok(thread)
}

pub fn handle_connect(
    callback: impl FnMut(&[u8]) -> i32 + 'static,
) -> Result<JoinHandle<()>, Box<dyn std::error::Error>> {
    let path_connect = Path::new("/sys/fs/bpf/my_prog/ringbuff_connect");

    let map = MapHandle::from_pinned_path(path_connect)?;
    let mut builder = RingBufferBuilder::new();
    builder.add(&map, callback)?;

    let ringbuf = builder.build()?;

    let thread = std::thread::spawn(move || {
        loop {
            let _ = ringbuf.poll(Duration::from_secs(5));
        }
    });
    Ok(thread)
}
