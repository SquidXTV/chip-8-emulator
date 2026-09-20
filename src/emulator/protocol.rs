use crate::emulator::display::{PIXEL_HEIGHT, PIXEL_WIDTH};

pub enum EmulationCommand {
    Load {
        bytes: Vec<u8>
    },
    Play,
    Pause,
    Step,
    Exit
}

pub struct EmulationFrame {
    pub pixels: [[bool; PIXEL_WIDTH]; PIXEL_HEIGHT]
}
