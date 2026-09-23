use crate::emulator::display::{FONT_BYTES_PER_CHARACTER, FONT_START_ADDRESS};

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
        self.registers[usize::from(register & 0x0F)]
    }

    pub fn next_instruction(&mut self) {
        self.program_counter += 2;
    }

    pub fn previous_instruction(&mut self) {
        self.program_counter -= 2;
    }

    pub fn jump_to(&mut self, address: u16) {
        self.program_counter = address;
    }

    pub fn jump_to_subroutine(&mut self, address: u16) {
        self.stack.push(self.program_counter);
        self.jump_to(address);
    }

    pub fn jump_with_offset(&mut self, address: u16) {
        // todo: push program counter to stack
        self.jump_to(address + u16::from(self.registers[0x0]))
    }

    pub fn return_from_subroutine(&mut self) {
        // todo: handle error properly
        let instruction_pre_subroutine = self.stack.pop().expect("Returning from subroutine only works when calling a subroutine before");
        self.jump_to(instruction_pre_subroutine);
    }

    pub fn skip_if_register_equals_value(&mut self, register: u8, value: u8) {
        if self.registers[usize::from(register & 0x0F)] == value {
            self.next_instruction();
        }
    }

    // todo: add proper bounds check and error if register index is not in u4
    pub fn skip_if_register_not_equals_value(&mut self, register: u8, value: u8) {
        if self.registers[usize::from(register & 0x0F)] != value {
            self.next_instruction();
        }
    }

    pub fn skip_if_register_equals_register(&mut self, register_a: u8, register_b: u8) {
        if self.registers[usize::from(register_a & 0x0F)] == self.registers[usize::from(register_b & 0x0F)] {
            self.next_instruction();
        }
    }

    pub fn skip_if_register_not_equals_register(&mut self, register_a: u8, register_b: u8) {
        if self.registers[usize::from(register_a & 0x0F)] != self.registers[usize::from(register_b & 0x0F)] {
            self.next_instruction();
        }
    }

    pub fn set_register_to_register(&mut self, register_lhs: u8, register_rhs: u8) {
        self.registers[usize::from(register_lhs & 0x0F)] = self.registers[usize::from(register_rhs & 0x0F)];
    }

    pub fn or(&mut self, register_lhs: u8, register_rhs: u8) {
        self.registers[usize::from(register_lhs & 0x0F)] |= self.registers[usize::from(register_rhs & 0x0F)];
    }

    pub fn and(&mut self, register_lhs: u8, register_rhs: u8) {
        self.registers[usize::from(register_lhs & 0x0F)] &= self.registers[usize::from(register_rhs & 0x0F)];
    }

    pub fn xor(&mut self, register_lhs: u8, register_rhs: u8) {
        self.registers[usize::from(register_lhs & 0x0F)] ^= self.registers[usize::from(register_rhs & 0x0F)];
    }

    pub fn add(&mut self, register_lhs: u8, register_rhs: u8) {
        let register_lhs = usize::from(register_lhs & 0x0F);
        let register_rhs = usize::from(register_rhs & 0x0F);

        let (sum, overflow) = self.registers[register_lhs].overflowing_add(self.registers[register_rhs]);

        self.registers[register_lhs] = sum;
        self.registers[0xF] = u8::from(overflow);
    }

    pub fn subtract(&mut self, register_lhs: u8, register_rhs: u8) {
        let register_lhs = usize::from(register_lhs & 0x0F);
        let register_rhs = usize::from(register_rhs & 0x0F);

        let (difference, overflow) = self.registers[register_lhs].overflowing_sub(self.registers[register_rhs]);

        self.registers[register_lhs] = difference;
        self.registers[0xF] = u8::from(!overflow);
    }

    pub fn subtract_reversed(&mut self, register_lhs: u8, register_rhs: u8) {
        let register_lhs = usize::from(register_lhs & 0x0F);
        let register_rhs = usize::from(register_rhs & 0x0F);

        let (difference, overflow) = self.registers[register_rhs].overflowing_sub(self.registers[register_lhs]);

        self.registers[register_lhs] = difference;
        self.registers[0xF] = u8::from(!overflow);
    }

    // todo: make both functions configurable for chip-48 and super-chip support
    pub fn shift_right(&mut self, target: u8, source: u8) {
        let target = usize::from(target & 0x0F);
        let source = usize::from(source & 0x0F);

        let value = self.registers[source];
        let carry = value & 0b0000_0001;

        self.registers[target] = value >> 1;
        self.registers[0xF] = carry;
    }

    pub fn shift_left(&mut self, target: u8, source: u8) {
        let target = usize::from(target & 0x0F);
        let source = usize::from(source & 0x0F);

        let value = self.registers[source];
        let carry = (value & 0b1000_0000) >> 7;

        self.registers[target] = value << 1;
        self.registers[0xF] = carry;
    }

    pub fn random(&mut self, target: u8, mask: u8) {
        self.registers[usize::from(target & 0x0F)] = rand::random::<u8>() & mask;
    }

    pub fn load_font_address_to_index_register(&mut self, character: u8) {
        let character = u16::from(self.at_register(character & 0x0F));
        let bytes_per_character = FONT_BYTES_PER_CHARACTER as u16;
        let character_address: u16 = FONT_START_ADDRESS + (character * bytes_per_character);
        self.set_index_register(character_address);
    }

    pub fn set_register(&mut self, register: u8, value: u8) {
        self.registers[usize::from(register & 0x0F)] = value;
    }

    pub fn set_index_register(&mut self, value: u16) {
        self.index_register = value;
    }

    pub fn add_to_index_register(&mut self, value: u16) {
        self.index_register += value;
    }

    pub fn add_to_register(&mut self, register: u8, value: u8) {
        self.registers[usize::from(register & 0x0F)] += value; // no carry flag set if overflow
    }
    
}
