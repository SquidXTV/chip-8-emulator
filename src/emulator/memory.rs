use crate::emulator::memory::MemoryError::RangeOutOfBounds;

pub const DEFAULT_SIZE: usize = 4 * 1024; // 4 kB

pub enum MemoryError {
    AddressOutOfBounds { address: u16 },
    RangeOutOfBounds { address: u16, size: usize },
}

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self::from_size(DEFAULT_SIZE)
    }

    pub fn from_size(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    // addresses are actually just referencing 12 bits (4 kB), but still stored in 16 bit references
    pub fn read(&self, address: u16) -> Result<&u8, MemoryError> {
        self.data
            .get(address as usize)
            .ok_or(MemoryError::AddressOutOfBounds { address })
    }

    pub fn read_slice(&self, address: u16, size: usize) -> Result<&[u8], MemoryError> {
        let start = address as usize;
        let Some(end) = start.checked_add(size) else {
            return Err(RangeOutOfBounds { address, size });
        };

        self.data
            .get(start..end)
            .ok_or(RangeOutOfBounds { address, size })
    }

    // opcodes / instructions are always 16 bits
    pub fn fetch_instruction(&self, address: u16) -> Result<u16, MemoryError> {
        self.read_slice(address, 2)
            .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError> {
        let Some(byte) = self.data.get_mut(address as usize) else {
            return Err(MemoryError::AddressOutOfBounds { address });
        };

        *byte = value;
        Ok(())
    }
}
