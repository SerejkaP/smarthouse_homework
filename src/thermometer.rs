use crate::room_devices::Device;

#[derive(Debug, Clone)]
pub struct Thermometer {
    temperature: f32,
    description: String,
}

impl Thermometer {
    pub fn build_thermometer(description: String) -> Self {
        Self {
            temperature: 0.0,
            description,
        }
    }

    pub fn show_themperature(&self) -> f32 {
        self.temperature
    }

    pub fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }
}

impl Device for Thermometer {
    fn info(&self) -> String {
        self.show_description().to_string() + " - " + self.temperature.to_string().as_str() + "°C"
    }

    fn show_description(&self) -> String {
        self.description.to_string()
    }
}
