use egui::debug_text::print;
use crate::emulator::cpu::{CPU, PROGRAM_START_ADDRESS};
use crate::emulator::display::Display;
use crate::emulator::keypad::Keypad;
use crate::emulator::memory::{Memory, MemoryError};
use crate::emulator::timer::Timer;

pub struct Chip {
    cpu: CPU,
    memory: Memory,
    display: Display,
    delay_timer: Timer,
    sound_timer: Timer,
    keypad: Keypad
}

impl Chip {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            memory: Memory::new(),
            display: Display::new(),
            delay_timer: Timer::new(),
            sound_timer: Timer::new(),
            keypad: Keypad::new()
        }
    }

    pub fn load(&mut self, bytes: &[u8]) -> Result<(), MemoryError> {
        // reset happens before writing, consider only resetting if write_slice was successful
        self.memory.reset();

        self.memory.write_slice(PROGRAM_START_ADDRESS, bytes).inspect(|_| {
            self.cpu.reset();
            self.display.reset();
            self.delay_timer.reset();
            self.sound_timer.reset();
        })
    }

    pub fn step(&mut self) -> Result<(), MemoryError> {
        let instruction = match self.memory.fetch_instruction(self.cpu.program_counter()) {
            Ok(instruction) => instruction,
            Err(error) => return Err(error),
        };

        self.cpu.increment_program_counter();
        self.execute(instruction);

        Ok(())
    }

    fn execute(&mut self, instruction: u16) {
        match instruction {
            0x00E0 => self.display.clear(),
            other => (println!("Not found instruction: {}", other)),
        }
    }

    // opcode: The first nibble. Used to divide instructions into broad categories
    // X: The second nibble. Used to look up one of the 16 registers (VX) from V0 through VF.
    // Y: The third nibble. Also used to look up one of the 16 registers (VY) from V0 through VF.
    // N: The fourth nibble. A 4-bit number.
    // NN: The second byte (third and fourth nibbles). An 8-bit immediate number.
    // NNN: The second, third and fourth nibbles. A 12-bit immediate memory address.
    fn extract_opcode(instruction: u16) -> u8 {
        (instruction >> 12) as u8
    }
}
