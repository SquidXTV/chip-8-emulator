use std::sync::mpsc;
use std::thread;
use crate::emulator::protocol::{EmulationCommand, EmulationFrame};

pub mod emulator;
pub mod ui;


#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let (command_sender, command_receiver) = mpsc::channel::<EmulationCommand>();
    let (frame_sender, frame_receiver) = mpsc::sync_channel::<EmulationFrame>(1);

    let emulation = thread::spawn(move || {
        emulator::run(command_receiver, frame_sender);
    });

    // run egui
    let _ = ui::run_application(&command_sender, frame_receiver);

    let _ = command_sender.send(EmulationCommand::Exit);
    let _ = emulation.join();
}