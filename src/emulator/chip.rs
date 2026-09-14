use crate::emulator::cpu::CPU;
use crate::emulator::display::Display;
use crate::emulator::keypad::Keypad;
use crate::emulator::memory::Memory;
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

    pub fn run(&mut self) {
        loop {
            let instruction = self.memory.fetch_instruction(self.cpu.program_counter);
            self.cpu.increment_program_counter();



            // todo: implement speed of ~700 iterations per second
        }
    }

    fn execute(&self, instruction: u16) {
        // match instruction {
        //     0x00E0 => self.display.clear()
        //     _ => ()
        // }
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