pub const PROGRAM_START_ADDRESS: u16 = 0x200;

pub struct CPU {
    program_counter: u16,
    index_register: u16,
    registers: [u8; 16],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            program_counter: PROGRAM_START_ADDRESS,
            index_register: 0,
            registers: [0; 16],
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn increment_program_counter(&mut self) {
        self.program_counter += 2;
    }
}
