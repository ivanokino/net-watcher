use eframe::egui;
use egui::Ui;

use crate::gui::{GuiApp, get_det_data};
pub fn render_socket(app: &mut GuiApp, ui: &mut Ui) {
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
            .show_rows(ui, 18.0, app.sockets.len(), |ui, row_range| {
                for idx in row_range {
                    let s = &app.sockets[idx];

                    let dropped_count = app.dropped_count;
                    let pid = s.pid;
                    let family = s.family_name();
                    let type_ = s.type_name();
                    let comm_name = s.comm_name().to_string();
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            [80.0, 18.0],
                            egui::Label::new(format!("{}", dropped_count + idx as u32 + 1)),
                        );
                        let mut response = ui.add_sized(
                            [80.0, 18.0],
                            egui::Label::new(pid.to_string()).sense(egui::Sense::click()),
                        );
                        if response.clicked() {
                            app.is_inf_click = true;
                            get_det_data(app, pid);
                        }

                        response = ui.add_sized(
                            [100.0, 18.0],
                            egui::Label::new(family).sense(egui::Sense::click()),
                        );
                        if response.clicked() {
                            app.is_inf_click = true;
                            get_det_data(app, pid);
                        }
                        response = ui.add_sized(
                            [100.0, 18.0],
                            egui::Label::new(type_).sense(egui::Sense::click()),
                        );
                        if response.clicked() {
                            app.is_inf_click = true;
                            get_det_data(app, pid);
                        }
                        response = ui.add_sized(
                            [100.0, 18.0],
                            egui::Label::new(comm_name).sense(egui::Sense::click()),
                        );
                        if response.clicked() {
                            app.is_inf_click = true;
                            get_det_data(app, pid);
                        }
                    });
                }
            });

        if app.is_inf_click {
            egui::Window::new("SOCKET").show(ui.ctx(), |ui| {
                if ui.button("close").clicked() {
                    app.is_inf_click = false;
                }

                ui.label(&app.selected_info);
            });
        }
    });
}

pub fn render_connect(app: &mut GuiApp, ui: &mut Ui) {
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
        .show_rows(ui, 18.0, app.connects.len(), |ui, row_range| {
            for idx in row_range {
                let dropped_count_connect = app.dropped_count_connect;
                let s = &app.connects[idx];
                let pid = s.pid;
                let fd = s.fd;
                let family = s.family_name();
                let port = s.port_name();

                let addr = s.addr_string();
                ui.horizontal(|ui| {
                    ui.add_sized(
                        [80.0, 18.0],
                        egui::Label::new(format!("{}", dropped_count_connect + idx as u32 + 1)),
                    );

                    let mut response = ui.add_sized(
                        [80.0, 18.0],
                        egui::Label::new(pid.to_string()).sense(egui::Sense::click()),
                    );
                    if response.clicked() {
                        app.is_inf_click = true;
                        get_det_data(app, pid);
                    }

                    response = ui.add_sized(
                        [100.0, 18.0],
                        egui::Label::new(fd.to_string()).sense(egui::Sense::click()),
                    );
                    if response.clicked() {
                        app.is_inf_click = true;
                        get_det_data(app, pid);
                    }
                    response = ui.add_sized(
                        [100.0, 18.0],
                        egui::Label::new(family).sense(egui::Sense::click()),
                    );
                    if response.clicked() {
                        app.is_inf_click = true;
                        get_det_data(app, pid);
                    }

                    response = ui.add_sized(
                        [200.0, 18.0],
                        egui::Label::new(addr).sense(egui::Sense::click()),
                    );
                    if response.clicked() {
                        app.is_inf_click = true;
                        get_det_data(app, pid);
                    }

                    response = ui.add_sized(
                        [100.0, 18.0],
                        egui::Label::new(port).sense(egui::Sense::click()),
                    );
                    if response.clicked() {
                        app.is_inf_click = true;
                        get_det_data(app, pid);
                    }
                });
            }
        });
    if app.is_inf_click {
        egui::Window::new("CONNECT").show(ui.ctx(), |ui| {
            if ui.button("close").clicked() {
                app.is_inf_click = false;
            }

            ui.label(&app.selected_info);
        });
    }
}
