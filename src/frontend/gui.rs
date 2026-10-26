use common::net::run_data_streaming;
use common::structs::{ConnGet, HandlerOutput, SockGet};

use common::structs::Tab;
use eframe::egui::{self};
use egui::Ui;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};

mod render_funcs;
use render_funcs::{render_connect, render_socket};

use crate::gui::render_funcs::render_bind;

pub struct GuiApp {
    pub current_tab: Tab,

    pub rx: Receiver<HandlerOutput>,
    pub sockets: VecDeque<SockGet>,
    pub connects: VecDeque<ConnGet>,
    pub binds: VecDeque<ConnGet>,
    pub dropped_count: u32,
    pub dropped_count_connect: u32,
    pub dropped_count_bind: u32,
    pub is_inf_click: bool,
    pub selected_info: String,
    pub det_stream: TcpStream,
}

impl eframe::App for GuiApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_tab, Tab::Socket, "SOCKET()");

            ui.selectable_value(&mut self.current_tab, Tab::Connect, "CONNECT");
            ui.selectable_value(&mut self.current_tab, Tab::Bind, "BIND");
        });
        while let Ok(data) = self.rx.try_recv() {
            let out_type: u8 = unsafe { data.conn_get.out_type };
            match out_type {
                0 => unsafe {
                    self.sockets.push_back(data.sock_get);
                    if self.sockets.len() > 20000 {
                        self.sockets.pop_front();
                        self.dropped_count += 1;
                    }
                },
                1 => unsafe {
                    self.connects.push_back(data.conn_get);
                    if self.connects.len() > 20000 {
                        self.connects.pop_front();
                        self.dropped_count_connect += 1;
                    }
                },
                _ => unsafe {
                    self.binds.push_back(data.bind_get);
                    if self.binds.len() > 20000 {
                        self.binds.pop_front();
                        self.dropped_count_bind += 1;
                    }
                },
            }
        }
        match self.current_tab {
            Tab::Socket => {
                ui.set_zoom_factor(2.0);

                render_socket(self, ui);
            }
            Tab::Connect => {
                render_connect(self, ui);
            }
            Tab::Bind => {
                render_bind(self, ui);
            }
        }
    }
}

pub fn run_gui_and_data_streaming(
    rx: Receiver<HandlerOutput>,
    det_stream: TcpStream,
    tx: Sender<HandlerOutput>,
    stream: TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([620.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MY APP",
        options,
        Box::new(|cc| {
            let ctx = cc.egui_ctx.clone();

            run_data_streaming(tx, stream, ctx);

            let app = GuiApp {
                current_tab: Tab::Socket,
                rx,
                sockets: VecDeque::new(),
                connects: VecDeque::new(),
                binds: VecDeque::new(),
                dropped_count: 0,
                dropped_count_connect: 0,
                dropped_count_bind: 0,
                is_inf_click: false,
                selected_info: "err".to_string(),
                det_stream,
            };
            Ok(Box::new(app))
        }),
    )?;

    Ok(())
}

fn get_det_data(app: &mut GuiApp, pid: u32) {
    let arr_bytes = pid.to_ne_bytes();
    let bytes = arr_bytes.as_slice();

    let _ = app.det_stream.write(bytes);

    let mut buff: [u8; 4096] = [0; 4096];

    let bytes_read = app.det_stream.read(&mut buff).unwrap_or(10);

    let inf = &mut buff[..bytes_read];

    for b in &mut *inf {
        if *b == 0 {
            *b = b' ';
        }
    }

    let string_inf = String::from_utf8_lossy(inf).into_owned();

    app.selected_info = string_inf;
    app.is_inf_click = true;
}
