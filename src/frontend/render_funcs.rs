use crate::gui::{GuiApp, get_det_data};
use eframe::egui;
use egui::Ui;
use egui_extras::{Column, TableBuilder};

pub fn render_socket(app: &mut GuiApp, ui: &mut Ui) {
    TableBuilder::new(ui)
        .striped(true)
        .sense(egui::Sense::click())
        .stick_to_bottom(true)
        .column(Column::exact(70.0)) // NUM
        .column(Column::exact(90.0)) // PID
        .column(Column::exact(100.0)) //FAMILY
        .column(Column::exact(100.0)) // TYPE
        .column(Column::exact(120.0)) //PROCESS NAME
        .header(24.0, |mut header| {
            header.col(|ui| {
                ui.strong("NUM");
            });
            header.col(|ui| {
                ui.strong("PID");
            });
            header.col(|ui| {
                ui.strong("FAMILY");
            });

            header.col(|ui| {
                ui.strong("TYPE");
            });
            header.col(|ui| {
                ui.strong("NAME");
            });
        })
        .body(|body| {
            body.rows(20.0, app.sockets.len(), |mut row| {
                let idx = row.index();
                let s = &app.sockets[idx];
                row.col(|ui| {
                    ui.label((app.dropped_count + idx as u32 + 1).to_string());
                });
                row.col(|ui| {
                    ui.label(s.pid.to_string());
                });
                row.col(|ui| {
                    ui.label(s.family_name());
                });
                row.col(|ui| {
                    ui.label(s.type_name());
                });

                row.col(|ui| {
                    ui.label(s.comm_name());
                });
                if row.response().clicked() {
                    app.is_inf_click = true;
                    get_det_data(app, s.pid);
                }
            });
        });
    if app.is_inf_click {
        egui::Window::new("SOCKET").show(ui.ctx(), |ui| {
            if ui.button("close").clicked() {
                app.is_inf_click = false;
            }

            ui.label(&app.selected_info);
        });
    }
}

pub fn render_connect(app: &mut GuiApp, ui: &mut Ui) {
    TableBuilder::new(ui)
        .striped(true)
        .sense(egui::Sense::click())
        .stick_to_bottom(true)
        .column(Column::exact(70.0)) // NUM
        .column(Column::exact(90.0)) // PID
        .column(Column::exact(65.0)) // FD
        .column(Column::exact(100.0)) //FAMILY
        .column(Column::exact(100.0)) // TYPE
        .column(Column::exact(70.0)) //PORT
        .column(Column::exact(120.0)) // PROCESS NAME
        .header(24.0, |mut header| {
            header.col(|ui| {
                ui.strong("NUM");
            });
            header.col(|ui| {
                ui.strong("PID");
            });
            header.col(|ui| {
                ui.strong("FD");
            });
            header.col(|ui| {
                ui.strong("FAMILY");
            });
            header.col(|ui| {
                ui.strong("ADDRESS");
            });
            header.col(|ui| {
                ui.strong("PORT");
            });
            header.col(|ui| {
                ui.strong("NAME");
            });
        })
        .body(|body| {
            body.rows(20.0, app.connects.len(), |mut row| {
                let idx = row.index();
                let s = &app.connects[idx];
                row.col(|ui| {
                    ui.label((app.dropped_count_connect + idx as u32 + 1).to_string());
                });
                row.col(|ui| {
                    ui.label(s.pid.to_string());
                });
                row.col(|ui| {
                    ui.label(s.fd.to_string());
                });
                row.col(|ui| {
                    ui.label(s.family_name());
                });

                row.col(|ui| {
                    ui.label(s.addr_string());
                });
                row.col(|ui| {
                    ui.label(s.port_name());
                });
                row.col(|ui| {
                    ui.label(s.comm_name());
                });
                if row.response().clicked() {
                    app.is_inf_click = true;
                    get_det_data(app, s.pid);
                }
            });
        });
    if app.is_inf_click {
        egui::Window::new("BIND").show(ui.ctx(), |ui| {
            if ui.button("close").clicked() {
                app.is_inf_click = false;
            }

            ui.label(&app.selected_info);
        });
    }
}
pub fn render_bind(app: &mut GuiApp, ui: &mut Ui) {
    TableBuilder::new(ui)
        .striped(true)
        .sense(egui::Sense::click())
        .stick_to_bottom(true)
        .column(Column::exact(70.0)) // NUM
        .column(Column::exact(90.0)) // PID
        .column(Column::exact(90.0)) // PID
        .column(Column::exact(100.0)) //FAMILY
        .column(Column::exact(100.0)) // TYPE
        .column(Column::exact(70.0)) //PORT
        .column(Column::exact(120.0)) // PROCESS NAME
        .header(24.0, |mut header| {
            header.col(|ui| {
                ui.strong("NUM");
            });
            header.col(|ui| {
                ui.strong("PID");
            });
            header.col(|ui| {
                ui.strong("FD");
            });

            header.col(|ui| {
                ui.strong("FAMILY");
            });
            header.col(|ui| {
                ui.strong("ADDRESS");
            });
            header.col(|ui| {
                ui.strong("PORT");
            });
            header.col(|ui| {
                ui.strong("NAME");
            });
        })
        .body(|body| {
            body.rows(20.0, app.binds.len(), |mut row| {
                let idx = row.index();
                let s = &app.binds[idx];
                row.col(|ui| {
                    ui.label((app.dropped_count_bind + idx as u32 + 1).to_string());
                });
                row.col(|ui| {
                    ui.label(s.pid.to_string());
                });
                row.col(|ui| {
                    ui.label(s.fd.to_string());
                });
                row.col(|ui| {
                    ui.label(s.family_name());
                });

                row.col(|ui| {
                    ui.label(s.addr_string());
                });
                row.col(|ui| {
                    ui.label(s.port_name());
                });
                row.col(|ui| {
                    ui.label(s.comm_name());
                });
                if row.response().clicked() {
                    app.is_inf_click = true;
                    get_det_data(app, s.pid);
                }
            });
        });
    if app.is_inf_click {
        egui::Window::new("BIND").show(ui.ctx(), |ui| {
            if ui.button("close").clicked() {
                app.is_inf_click = false;
            }

            ui.label(&app.selected_info);
        });
    }
}
