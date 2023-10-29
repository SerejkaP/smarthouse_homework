use crate::room_devices::Device;

#[derive(Debug, Clone)]
pub struct Socket {
    power: u32,
    description: String,
    on: bool,
}

impl Socket {
    pub fn build_socket(description: String) -> Self {
        Self {
            power: 0,
            description,
            on: false,
        }
    }

    pub fn show_power(&self) -> u32 {
        self.power
    }

    pub fn turn_on(&mut self, on: bool) {
        self.on = on;
    }

    pub fn set_power(&mut self, power: u32) {
        self.power = power;
    }
}

impl Device for Socket {
    fn info(&self) -> String {
        self.show_description().to_string()
            + " - "
            + self.show_power().to_string().as_str()
            + "volt ("
            + if self.on { "ON" } else { "OFF" }
            + ")"
    }

    fn show_description(&self) -> String {
        self.description.to_string()
    }
}
