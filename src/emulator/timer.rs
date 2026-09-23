pub struct Timer {
    counter: u8,
}

// todo: make more generic by accepting counter start, callback what happens when reaching 0, etc
impl Timer {
    pub fn new() -> Self {
        Self { counter: u8::MAX }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn counter(&self) -> u8 {
        self.counter
    }

    pub fn set_counter(&mut self, value: u8) {
        self.counter = value;
    }

}
