use crate::emulator::display;

pub const PROGRAM_START_ADDRESS: u16 = 0x200;

pub struct CPU {
    program_counter: u16,
    index_register: u16,
    registers: [u8; 16],
    stack: Vec<u16>
}

impl CPU {
    pub fn new() -> Self {
        Self {
            program_counter: PROGRAM_START_ADDRESS,
            index_register: 0,
            registers: [0; 16],
            stack: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn index_register(&self) -> u16 {
        self.index_register
    }

    pub fn at_register(&self, register: u8) -> u8 {
        self.registers[register & 0x0F]
    }

    pub fn next_instruction(&mut self) {
        self.program_counter += 2;
    }

    pub fn jump_to(&mut self, address: u16) {
        self.program_counter = address;
    }

    pub fn jump_to_subroutine(&mut self, address: u16) {
        self.stack.push(self.program_counter);
        self.jump_to(address);
    }

    pub fn return_from_subroutine(&mut self) {
        // todo: handle error properly
        self.jump_to(self.stack.pop().expect("Returning from subroutine only works when calling a subroutine before"));
    }

    pub fn skip_if_register_equals_value(&mut self, register: u8, value: u8) {
        if self.registers[register & 0x0F] == value {
            self.next_instruction();
        }
    }

    // todo: add proper bounds check and error if register index is not in u4
    pub fn skip_if_register_not_equals_value(&mut self, register: u8, value: u8) {
        if self.registers[register & 0x0F] != value {
            self.next_instruction();
        }
    }

    pub fn set_register(&mut self, register: u8, value: u8) {
        self.registers[register & 0x0F] = value;
    }

    pub fn set_index_register(&mut self, value: u16) {
        self.index_register = value;
    }

    pub fn add_to_register(&mut self, register: u8, value: u8) {
        self.registers[register & 0x0F] += value; // no carry flag set if overflow
    }
    
}
