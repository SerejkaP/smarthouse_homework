pub struct Socket {
    _power: u32,
    description: String,
    _on: bool,
}

impl Socket {
    pub fn build_socket(description: String) -> Self {
        Self {
            _power: 0,
            description,
            _on: false,
        }
    }

    pub fn _show_power(&self) -> u32 {
        self._power
    }

    pub fn show_description(&self) -> &str {
        &self.description
    }

    pub fn _turn_on(&mut self, on: bool) {
        self._on = on;
    }
}
