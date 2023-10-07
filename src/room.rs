pub struct Room {
    pub description: String,
    devices: Vec<String>,
}

impl Room {
    pub fn new(description: String, devices: Vec<String>) -> Self {
        Self {
            description,
            devices,
        }
    }

    pub fn show_description(&self) -> &str {
        &self.description
    }

    pub fn show_devices(&self) -> &Vec<String> {
        &self.devices
    }
}
