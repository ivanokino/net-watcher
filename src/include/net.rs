#![allow(dead_code)]

use eframe::egui::Context;
use std::net::{self, TcpStream};
use std::ptr::read_unaligned;
use std::sync::mpsc::Sender;

use crate::structs::SockGet;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::thread;
use std::time::Duration;

pub fn connect_to_handler() -> (TcpStream, TcpStream) {
    let stream = loop {
        match net::TcpStream::connect("127.0.0.1:8085") {
            Ok(x) => break x,
            Err(_) => std::thread::sleep(Duration::from_secs(2)),
        };
    };

    let det_stream = loop {
        match net::TcpStream::connect("127.0.0.1:8086") {
            Ok(x) => break x,
            Err(_) => std::thread::sleep(Duration::from_secs(2)),
        };
    };

    (stream, det_stream)
}

pub fn run_detail_streaming(mut det_stream: TcpStream) {
    thread::spawn(move || {
        loop {
            let mut buff: [u8; 4] = [0; 4];

            match det_stream.read_exact(&mut buff) {
                Ok(_) => (),
                Err(_) => break,
            };

            let pid = u32::from_ne_bytes(buff);

            let inf = get_info(pid);
            let bytes = inf.as_bytes();

            let _ = det_stream.write(bytes);
        }
    });
}

fn get_info(pid: u32) -> String {
    let command_text = match fs::read_to_string(format!("/proc/{}/cmdline", pid)) {
        Ok(x) => x,
        Err(_) => String::from("err"),
    };

    let path_text = match fs::read_link(format!("/proc/{}/exe", pid)) {
        Ok(x) => x.to_string_lossy().into_owned(),
        Err(_) => String::from("err"),
    };

    let status = match fs::read_to_string(format!("/proc/{}/status", pid)) {
        Ok(x) => x,
        Err(_) => String::from("err"),
    };

    let mut ppid = String::from("err");

    for line in status.lines() {
        if let Some(x) = line.strip_prefix("PPid:") {
            ppid = x.trim().to_string();
        }
    }

    let final_output = format!(
        "COMMAND: {}\n\nPATH: {}\n\nPPID: {}",
        command_text, path_text, ppid
    );

    final_output
}

pub fn run_data_streaming(tx: Sender<SockGet>, mut stream: TcpStream, ctx: Context) -> u8 {
    std::thread::spawn(move || {
        loop {
            let mut buf = [0; size_of::<SockGet>()];

            match stream.read_exact(&mut buf) {
                Ok(()) => {}
                Err(_) => break,
            };
            let data: SockGet = unsafe { read_unaligned(buf.as_ptr() as *const SockGet) };
            // println!("send data");
            let _ = tx.send(data);
            ctx.request_repaint();
        }
    });

    0
}
