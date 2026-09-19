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

        self.cpu.next_instruction();
        self.execute(instruction);

        Ok(())
    }

    fn execute(&mut self, instruction: u16) {
        match instruction {
            0x00E0 => self.display.clear(),
            0x1000..=0x1FFF => self.cpu.jump_to(instruction & 0x0FFF),
            0x2000..=0x2FFF => self.cpu.jump_to_subroutine(instruction & 0x0FFF),
            0x00EE => self.cpu.return_from_subroutine(),
            0x3000..=0x3FFF => self.cpu.skip_if_register_equals_value(Self::extract_x(instruction), Self::extract_nn(instruction)),
            0x6000..=0x6FFF => self.cpu.set_register(Self::extract_x(instruction), Self::extract_nn(instruction)),
            0x7000..=0x7FFF => self.cpu.add_to_register(Self::extract_x(instruction), Self::extract_nn(instruction)),
            0xA000..=0xAFFF => self.cpu.set_index_register(Self::extract_nnn(instruction)),
            0xD000..=0xDFFF => {
                let x = self.cpu.at_register(Self::extract_x(instruction));
                let y = self.cpu.at_register(Self::extract_y(instruction));
                // todo: handle error properly
                // let Some(sprite) = self.memory.read_slice(self.cpu.index_register(), usize::from(Self::extract_n(instruction))) else { () };
                let sprite = self.memory.read_slice(self.cpu.index_register(), usize::from(Self::extract_n(instruction)));

                if let Ok(sprite) = sprite {
                    self.display.draw_sprite(x, y, sprite);
                }

            },
            other => println!("Not found instruction: 0x{:04X}", other),
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

    fn extract_x(instruction: u16) -> u8 {
        ((instruction & 0x0F00) >> 8) as u8
    }

    fn extract_y(instruction: u16) -> u8 {
        ((instruction & 0x00F0) >> 4) as u8
    }

    fn extract_n(instruction: u16) -> u8 {
        (instruction & 0x000F) as u8
    }

    fn extract_nn(instruction: u16) -> u8 {
        (instruction & 0x00FF) as u8
    }

    fn extract_nnn(instruction: u16) -> u16 {
        instruction & 0x0FFF
    }

}
