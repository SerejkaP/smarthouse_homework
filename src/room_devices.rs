use super::socket::Socket;
use super::thermometer::Thermometer;

#[derive(Debug, Clone)]
pub enum RoomDevices {
    Socket(Socket),
    Thermometer(Thermometer),
}

pub trait Device {
    fn info(&self) -> String;
    fn show_description(&self) -> String;
}

impl Device for RoomDevices {
    fn info(&self) -> String {
        match self {
            Self::Socket(socket) => socket.info(),
            Self::Thermometer(thermometer) => thermometer.info(),
        }
    }
    fn show_description(&self) -> String {
        match self {
            Self::Socket(socket) => socket.show_description().to_string(),
            Self::Thermometer(thermometer) => thermometer.show_themperature().to_string(),
        }
    }
}
