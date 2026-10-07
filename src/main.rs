#[path = "./frontend/gui.rs"]
mod gui;

use common::net::connect_to_handler;

use std::boxed::Box;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = std::sync::mpsc::channel();

    let (stream, det_stream) = connect_to_handler();

    stream.set_nodelay(true)?;

    gui::run_gui_and_data_streaming(rx, det_stream, tx, stream)?;

    Ok(())
}
