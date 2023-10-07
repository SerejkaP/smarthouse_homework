pub struct Thermometer {
    temperature: f32,
}

impl Thermometer {
    pub fn build_thermometer() -> Self {
        Self { temperature: 0.0 }
    }

    pub fn show_themperature(&self) -> f32 {
        self.temperature
    }
}
