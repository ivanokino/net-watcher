use common::net::run_data_streaming;
use common::structs::ConnGet;
use common::structs::HandlerOutput;
use common::structs::SockGet;

use common::structs::Tab;
use eframe::egui::{self};
use egui::Ui;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};

pub struct GuiApp {
    current_tab: Tab,

    pub rx: Receiver<HandlerOutput>,
    pub sockets: VecDeque<SockGet>,
    pub connects: VecDeque<ConnGet>,
    pub dropped_count: u32,
    pub dropped_count_connect: u32,
    pub is_inf_click: bool,
    pub selected_info: String,
    pub det_stream: TcpStream,
}

impl eframe::App for GuiApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_tab, Tab::Socket, "SOCKET()");

            ui.selectable_value(&mut self.current_tab, Tab::Connect, "CONNECT");
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
                _ => unsafe {
                    self.connects.push_back(data.conn_get);
                    if self.connects.len() > 20000 {
                        self.connects.pop_front();
                        self.dropped_count_connect += 1;
                    }
                },
            }
        }
        match self.current_tab {
            Tab::Socket => {
                ui.set_zoom_factor(2.0);
                egui::CentralPanel::default().show(ui, |ui| {
                    // ui.input(|i| i.smooth_scroll_delta) й

                    ui.horizontal(|ui| {
                        ui.add_sized([80.0, 18.0], egui::Label::new("NUM"));
                        ui.add_sized([80.0, 18.0], egui::Label::new("PID"));
                        ui.add_sized([100.0, 18.0], egui::Label::new("FAMILY"));

                        ui.add_sized([100.0, 18.0], egui::Label::new("TYPE"));
                        ui.add_sized([100.0, 18.0], egui::Label::new("NAME"));
                    });
                    egui::ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .auto_shrink([false, false])
                        .show_rows(ui, 18.0, self.sockets.len(), |ui, row_range| {
                            for idx in row_range {
                                let s = &self.sockets[idx];

                                let dropped_count = self.dropped_count;
                                let pid = s.pid;
                                let family = s.family_name();
                                let type_ = s.type_name();
                                let comm_name = s.comm_name().to_string();
                                ui.horizontal(|ui| {
                                    ui.add_sized(
                                        [80.0, 18.0],
                                        egui::Label::new(format!(
                                            "{}",
                                            dropped_count + idx as u32 + 1
                                        )),
                                    );
                                    let mut response = ui.add_sized(
                                        [80.0, 18.0],
                                        egui::Label::new(pid.to_string())
                                            .sense(egui::Sense::click()),
                                    );
                                    if response.clicked() {
                                        self.is_inf_click = true;
                                        get_det_data(self, pid);
                                    }

                                    response = ui.add_sized(
                                        [100.0, 18.0],
                                        egui::Label::new(family).sense(egui::Sense::click()),
                                    );
                                    if response.clicked() {
                                        self.is_inf_click = true;
                                        get_det_data(self, pid);
                                    }
                                    response = ui.add_sized(
                                        [100.0, 18.0],
                                        egui::Label::new(type_).sense(egui::Sense::click()),
                                    );
                                    if response.clicked() {
                                        self.is_inf_click = true;
                                        get_det_data(self, pid);
                                    }
                                    response = ui.add_sized(
                                        [100.0, 18.0],
                                        egui::Label::new(comm_name).sense(egui::Sense::click()),
                                    );
                                    if response.clicked() {
                                        self.is_inf_click = true;
                                        get_det_data(self, pid);
                                    }
                                });
                            }
                        });

                    if self.is_inf_click {
                        egui::Window::new("SOCKET").show(ui.ctx(), |ui| {
                            if ui.button("close").clicked() {
                                self.is_inf_click = false;
                            }

                            ui.label(&self.selected_info);
                        });
                    }
                });
            }

            Tab::Connect => {
                ui.horizontal(|ui| {
                    ui.add_sized([80.0, 18.0], egui::Label::new("NUM"));
                    ui.add_sized([80.0, 18.0], egui::Label::new("PID"));
                    ui.add_sized([100.0, 18.0], egui::Label::new("FD"));

                    ui.add_sized([100.0, 18.0], egui::Label::new("FAMILY"));
                    ui.add_sized([200.0, 18.0], egui::Label::new("ADDRESS"));

                    ui.add_sized([100.0, 18.0], egui::Label::new("PORT"));
                });

                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .auto_shrink([false, false])
                    .show_rows(ui, 18.0, self.connects.len(), |ui, row_range| {
                        for idx in row_range {
                            let dropped_count_connect = self.dropped_count_connect;
                            let s = &self.connects[idx];
                            let pid = s.pid;
                            let fd = s.fd;
                            let family = s.family_name();
                            let port = s.port_name();

                            let addr = s.addr_string();
                            ui.horizontal(|ui| {
                                ui.add_sized(
                                    [80.0, 18.0],
                                    egui::Label::new(format!(
                                        "{}",
                                        dropped_count_connect + idx as u32 + 1
                                    )),
                                );

                                let mut response = ui.add_sized(
                                    [80.0, 18.0],
                                    egui::Label::new(pid.to_string()).sense(egui::Sense::click()),
                                );
                                if response.clicked() {
                                    self.is_inf_click = true;
                                    get_det_data(self, pid);
                                }

                                response = ui.add_sized(
                                    [100.0, 18.0],
                                    egui::Label::new(fd.to_string()).sense(egui::Sense::click()),
                                );
                                if response.clicked() {
                                    self.is_inf_click = true;
                                    get_det_data(self, pid);
                                }
                                response = ui.add_sized(
                                    [100.0, 18.0],
                                    egui::Label::new(family).sense(egui::Sense::click()),
                                );
                                if response.clicked() {
                                    self.is_inf_click = true;
                                    get_det_data(self, pid);
                                }

                                response = ui.add_sized(
                                    [200.0, 18.0],
                                    egui::Label::new(addr).sense(egui::Sense::click()),
                                );
                                if response.clicked() {
                                    self.is_inf_click = true;
                                    get_det_data(self, pid);
                                }

                                response = ui.add_sized(
                                    [100.0, 18.0],
                                    egui::Label::new(port).sense(egui::Sense::click()),
                                );
                                if response.clicked() {
                                    self.is_inf_click = true;
                                    get_det_data(self, pid);
                                }
                            });
                        }
                    });
                if self.is_inf_click {
                    egui::Window::new("CONNECT").show(ui.ctx(), |ui| {
                        if ui.button("close").clicked() {
                            self.is_inf_click = false;
                        }

                        ui.label(&self.selected_info);
                    });
                }
            }
        }
    }
}

//ui.request_repaint_after(Duration::from_millis(35));

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
                dropped_count: 0,
                dropped_count_connect: 0,
                is_inf_click: false,
                selected_info: "test".to_string(),
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
