use std::ops::Add;

use super::socket::*;
use super::thermometer::*;

pub trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn info(&self) -> String;
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
pub struct OwningDeviceInfoProvider {
    pub socket: Socket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn info(&self) -> String {
        "Devices: ".to_owned().add(self.socket.show_description())
    }
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a Socket,
    pub thermo: &'b Thermometer,
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn info(&self) -> String {
        let socket_report = "Devices: ".to_owned().add(self.socket.show_description());
        let thermometer_report =
            "; Themperature: ".to_owned() + &self.thermo.show_themperature().to_string();
        socket_report.add(&thermometer_report)
    }
}
