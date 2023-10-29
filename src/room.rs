use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::{room_devices::Device, socket::Socket, thermometer::Thermometer};

use super::room_devices::RoomDevices;

#[derive(Debug, Clone)]
pub struct Room {
    pub description: String,
    pub devices: HashMap<String, RoomDevices>,
}

impl Room {
    pub fn new(description: String, devices: Vec<RoomDevices>) -> Self {
        Self {
            description,
            devices: devices
                .into_iter()
                .map(|d| (d.show_description(), d))
                .collect(),
        }
    }

    pub fn show_description(&self) -> &str {
        &self.description
    }

    pub fn info(&self) -> Vec<String> {
        let mut room_info: Vec<String> = Vec::new();
        self.devices.iter().for_each(|d| {
            room_info.push(d.1.info().to_string());
        });
        room_info
    }

    pub fn add_device(&mut self) {
        io::stdout()
            .write_all(b"Write device type (socket or thermometer)\n")
            .unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        match buffer.to_lowercase().trim() {
            "socket" => {
                io::stdout().write_all(b"Write socket name\n").unwrap();
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
                let device = Socket::build_socket(buffer.trim().to_string());
                self.devices
                    .insert(device.show_description(), RoomDevices::Socket(device));
            }
            "thermometer" => {
                io::stdout().write_all(b"Write thermometer name\n").unwrap();
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
                let device = Thermometer::build_thermometer(buffer.trim().to_string());
                self.devices
                    .insert(device.show_description(), RoomDevices::Thermometer(device));
            }
            _ => io::stdout()
                .write_all(b"Wrong device type (socket or thermometer)!\n")
                .unwrap(),
        }
    }

    pub fn remove_device(&mut self) {
        io::stdout()
            .write_all(b"Write device name to remove\n")
            .unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        self.devices.remove(buffer.trim());
    }

    pub fn devices(&self) -> Vec<&str> {
        self.devices.iter().map(|d| d.0.as_str()).collect()
    }
}
