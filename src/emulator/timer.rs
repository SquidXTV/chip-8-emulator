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
}
