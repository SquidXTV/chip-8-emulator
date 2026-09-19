use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};
use std::thread;
use std::time::Duration;
use crate::emulator::chip::Chip;
use crate::emulator::protocol::{EmulationCommand, EmulationFrame};

mod cpu;
mod memory;
mod timer;
mod chip;
mod keypad;
pub mod display;
pub mod protocol;

pub fn run(commands: Receiver<EmulationCommand>, frames: SyncSender<EmulationFrame>) {
    let mut chip = Chip::new();
    let mut running = false;

    loop {

        loop {
            match commands.try_recv() {
                Ok(EmulationCommand::Load { bytes }) => {
                    let _ = chip.load(&bytes); // todo: handle memory error
                },
                Ok(EmulationCommand::Play) => {
                    running = true;
                },
                Ok(EmulationCommand::Pause) => {
                    running = false;
                },
                Ok(EmulationCommand::Step) => {
                    let _ = chip.step(); // todo: handle memory error
                },
                Ok(EmulationCommand::Exit) => {
                    return;
                },
                Err(TryRecvError::Disconnected) => {
                    return;
                },
                Err(TryRecvError::Empty) => {
                    break;
                },
            }
        }

        if running {
            for _ in 0..10 {
                let _ = chip.step(); // todo: handle memory error
            }

            // frames.try_send(EmulationFrame {
            //     pixels: ...
            // });
        }

        thread::sleep(Duration::from_millis(250));
    }
}
