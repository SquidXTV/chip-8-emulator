pub enum EmulationCommand {
    Load {
        bytes: [u8]
    },
    Play,
    Pause,
    Step,
    Exit
}

pub struct EmulationFrame {
    pub pixels: Vec<bool>
}
