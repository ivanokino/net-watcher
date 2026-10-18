use std::net::TcpListener;
use std::{
    io::Write,
    mem,
    sync::{Arc, Mutex},
};

use common::bpf::{handle_connect, handle_socket};
use common::structs::HandlerOutput;

use common::net::run_detail_streaming;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8085")?;
    let (stream, _) = listener.accept()?;
    println!("back connected");

    let det_listener = TcpListener::bind("127.0.0.1:8086")?;
    let (det_stream, _) = det_listener.accept()?; //stream for detail information 

    let shared_stream = Arc::new(Mutex::new(stream));
    let socket_stream_clone = shared_stream.clone();
    let connect_stream_clone = shared_stream.clone();

    run_detail_streaming(det_stream);

    let socket_callback = move |data: &[u8]| -> i32 {
        if data.len() < mem::size_of::<HandlerOutput>() {
            println!("bad data from buffer (socket)");
            return 0;
        }

        let mut buff = [0; mem::size_of::<HandlerOutput>()];

        buff[..data.len()].copy_from_slice(data);

        let mut guard = socket_stream_clone.lock().expect("mutex poisoned");
        let _ = guard.write_all(buff.as_slice());
        let _ = guard.flush();

        0
    };

    let connect_callback = move |data: &[u8]| -> i32 {
        /*     if data.len() < mem::size_of::<HandlerOutput>() {
                    return 0;
                };
        */

        let mut buff = [0; mem::size_of::<HandlerOutput>()];
        buff[..data.len()].copy_from_slice(data);

        let mut guard = connect_stream_clone.lock().expect("mutex poisoned");
        let _ = guard.write_all(buff.as_slice());
        let _ = guard.flush();
        0
    };

    let socket_thread = handle_socket(socket_callback)?;
    let connect_thread = handle_connect(connect_callback)?;
    let _ = socket_thread.join();
    let _ = connect_thread.join();
    Ok(())
    //handle_connect(connect_callback)
}
