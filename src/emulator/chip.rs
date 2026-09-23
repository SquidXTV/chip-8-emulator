use std::sync::{Arc, Mutex};
use crate::emulator::cpu::{CPU, PROGRAM_START_ADDRESS};
use crate::emulator::display::{Display, FONT, FONT_START_ADDRESS};
use crate::emulator::keypad::Keypad;
use crate::emulator::memory::{Memory, MemoryError};
use crate::emulator::protocol::EmulationFrame;
use crate::emulator::timer::Timer;

pub struct Chip {
    cpu: CPU,
    memory: Memory,
    display: Display,
    delay_timer: Timer,
    sound_timer: Timer,
    keypad: Keypad,
    current_frame: Arc<Mutex<EmulationFrame>>
}

impl Chip {
    pub fn new(current_frame: Arc<Mutex<EmulationFrame>>) -> Self {
        let mut chip = Self {
            cpu: CPU::new(),
            memory: Memory::new(),
            display: Display::new(),
            delay_timer: Timer::new(),
            sound_timer: Timer::new(),
            keypad: Keypad::new(),
            current_frame
        };

        let _ = chip.memory.write_slice(FONT_START_ADDRESS, FONT.as_flattened()); // todo: handle memory error

        chip
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
            0x00E0 => {
                self.display.clear();
                self.send_display();
            },
            0x1000..=0x1FFF => self.cpu.jump_to(Self::extract_nnn(instruction)),
            0x2000..=0x2FFF => self.cpu.jump_to_subroutine(Self::extract_nnn(instruction)),
            0x00EE => self.cpu.return_from_subroutine(),
            0x3000..=0x3FFF => self.cpu.skip_if_register_equals_value(Self::extract_x(instruction), Self::extract_nn(instruction)),
            0x4000..=0x4FFF => self.cpu.skip_if_register_not_equals_value(Self::extract_x(instruction), Self::extract_nn(instruction)),
            instruction if instruction & 0xF00F == 0x5000 => self.cpu.skip_if_register_equals_register(Self::extract_x(instruction), Self::extract_y(instruction)),
            instruction if instruction & 0xF00F == 0x9000 => self.cpu.skip_if_register_not_equals_register(Self::extract_x(instruction), Self::extract_y(instruction)),
            0x6000..=0x6FFF => self.cpu.set_register(Self::extract_x(instruction), Self::extract_nn(instruction)),
            0x7000..=0x7FFF => self.cpu.add_to_register(Self::extract_x(instruction), Self::extract_nn(instruction)),
            instruction if instruction & 0xF00F == 0x8000 => self.cpu.set_register_to_register(Self::extract_x(instruction), Self::extract_y(instruction)),
            instruction if instruction & 0xF00F == 0x8001 => self.cpu.or(Self::extract_x(instruction), Self::extract_y(instruction)),
            instruction if instruction & 0xF00F == 0x8002 => self.cpu.and(Self::extract_x(instruction), Self::extract_y(instruction)),
            instruction if instruction & 0xF00F == 0x8003 => self.cpu.xor(Self::extract_x(instruction), Self::extract_y(instruction)),
            instruction if instruction & 0xF00F == 0x8004 => self.cpu.add(Self::extract_x(instruction), Self::extract_y(instruction)),
            instruction if instruction & 0xF00F == 0x8005 => self.cpu.subtract(Self::extract_x(instruction), Self::extract_y(instruction)),
            instruction if instruction & 0xF00F == 0x8006 => self.cpu.shift_right(Self::extract_x(instruction), Self::extract_y(instruction)),
            instruction if instruction & 0xF00F == 0x8007 => self.cpu.subtract_reversed(Self::extract_x(instruction), Self::extract_y(instruction)),
            instruction if instruction & 0xF00F == 0x800E => self.cpu.shift_left(Self::extract_x(instruction), Self::extract_y(instruction)),
            0xA000..=0xAFFF => self.cpu.set_index_register(Self::extract_nnn(instruction)),
            0xB000..=0xBFFF => self.cpu.jump_with_offset(Self::extract_nnn(instruction)),
            0xC000..=0xCFFF => self.cpu.random(Self::extract_x(instruction), Self::extract_nn(instruction)),
            0xD000..=0xDFFF => {
                let x = self.cpu.at_register(Self::extract_x(instruction));
                let y = self.cpu.at_register(Self::extract_y(instruction));
                // todo: handle error properly
                // let Some(sprite) = self.memory.read_slice(self.cpu.index_register(), usize::from(Self::extract_n(instruction))) else { () };
                let sprite = self.memory.read_slice(self.cpu.index_register(), usize::from(Self::extract_n(instruction)));

                if let Ok(sprite) = sprite {
                    let updated = self.display.draw_sprite(usize::from(x), usize::from(y), sprite);

                    if updated {
                        self.cpu.set_register(0xF, 0x1);
                        self.send_display();
                    } else {
                        self.cpu.set_register(0xF, 0x0);
                    }
                }

            },
            instruction if instruction & 0xF0FF == 0xE09E => {
                let target_key = self.cpu.at_register(Self::extract_x(instruction)) & 0x0F;

                // todo: switch to 'is_pressed' check when keypad supports multiple keys
                if self.keypad.current_key() == Some(target_key) {
                    self.cpu.next_instruction();
                }
            },
            instruction if instruction & 0xF0FF == 0xE0A1 => {
                let target_key = self.cpu.at_register(Self::extract_x(instruction)) & 0x0F;

                // todo: switch to 'is_pressed' check when keypad supports multiple keys
                if self.keypad.current_key() != Some(target_key) {
                    self.cpu.next_instruction();
                }
            },
            instruction if instruction & 0xF0FF == 0xF007 => self.cpu.set_register(Self::extract_x(instruction), self.delay_timer.counter()),
            instruction if instruction & 0xF0FF == 0xF015 => self.delay_timer.set_counter(self.cpu.at_register(Self::extract_x(instruction))),
            instruction if instruction & 0xF0FF == 0xF018 => self.sound_timer.set_counter(self.cpu.at_register(Self::extract_x(instruction))),
            instruction if instruction & 0xF0FF == 0xF01E => self.cpu.add_to_index_register(u16::from(Self::extract_x(instruction))),
            instruction if instruction & 0xF0FF == 0xF00A => {
                match self.keypad.current_key() {
                    None => {
                        self.cpu.previous_instruction();
                    }
                    Some(key) => {
                        // todo: fix keypad to support pressing multiple keys at the same time and 'popping' the latest pressed key for this one
                        self.cpu.set_register(Self::extract_x(instruction), key & 0x0F);
                    }
                }
            },
            instruction if instruction & 0xF0FF == 0xF029 => self.cpu.load_font_address_to_index_register(Self::extract_x(instruction)),
            instruction if instruction & 0xF0FF == 0xF033 => {
                let source = self.cpu.at_register(Self::extract_x(instruction));
                let target_address = self.cpu.index_register();

                let digits = [
                    source / 100,
                    source / 10 % 10,
                    source % 10
                ];

                // todo: handle memory error
                let _ = self.memory.write_slice(target_address, &digits);
            },
            instruction if instruction & 0xF0FF == 0xF055 => {
                let address = self.cpu.index_register();
                let to_register = Self::extract_x(instruction) & 0x0F;

                let registers: Vec<u8> = (0..=to_register)
                    .map(|register| self.cpu.at_register(register))
                    .collect();

                let _ = self.memory.write_slice(address, &registers);
                // todo: make configurable for old chip8 behavior of increasing the actual index register
                // self.cpu.set_index_register(address + u16::from(to_register) + 1);
            },
            instruction if instruction & 0xF0FF == 0xF065 => {
                let address = self.cpu.index_register();
                let end = usize::from(Self::extract_x(instruction) & 0x0F);

                // todo: handle memory error
                if let Ok(bytes) = self.memory.read_slice(address, end + 1) {
                    for (register, &byte) in bytes.iter().enumerate() {
                        self.cpu.set_register(register as u8, byte);
                    }
                }
            },
            other => println!("Not found instruction: 0x{:04X}", other),
        }

        println!("Executed instruction: 0x{:04X}", instruction);
    }

    // opcode: The first nibble. Used to divide instructions into broad categories
    // X: The second nibble. Used to look up one of the 16 registers (VX) from V0 through VF.
    // Y: The third nibble. Also used to look up one of the 16 registers (VY) from V0 through VF.
    // N: The fourth nibble. A 4-bit number.
    // NN: The second byte (third and fourth nibbles). An 8-bit immediate number.
    // NNN: The second, third and fourth nibbles. A 12-bit immediate memory address.
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

    fn send_display(&self) {
        // todo: handle error state
        let mut frame = self.current_frame.lock().expect("...");
        *frame = EmulationFrame {
            pixels: self.display.pixels()
        };
    }

}
