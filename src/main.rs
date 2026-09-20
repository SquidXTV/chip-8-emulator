use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use crate::emulator::protocol::{EmulationCommand, EmulationFrame};

pub mod emulator;
pub mod ui;


#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let (command_sender, command_receiver) = mpsc::channel::<EmulationCommand>();

    let current_frame: Arc<Mutex<EmulationFrame>> = Arc::new(Mutex::new(EmulationFrame {pixels: [[false; 64]; 32]}));
    let emulation_frame = Arc::clone(&current_frame);

    let emulation = thread::spawn(move || {
        emulator::run(command_receiver, emulation_frame);
    });

    // run egui
    let _ = ui::run_application(command_sender.clone(), current_frame);

    let _ = command_sender.send(EmulationCommand::Exit);
    let _ = emulation.join();
}