use std::collections::HashMap;

use smarthouse_homework::prelude::*;

#[test]
fn test_house() {
    let socket1 = Socket::build_socket(String::from("Socket1"));
    let socket2 = Socket::build_socket(String::from("Socket2"));
    let thermo1 = Thermometer::build_thermometer(String::from("Thermometer1"));
    let thermo2 = Thermometer::build_thermometer(String::from("Thermometer2"));

    // Инициализация дома
    let devices1: Vec<RoomDevices> = vec![
        RoomDevices::Socket(socket1),
        RoomDevices::Thermometer(thermo1),
    ];
    let room1 = Room::new(String::from("Room1"), devices1);

    let devices2: Vec<RoomDevices> = vec![
        RoomDevices::Socket(socket2),
        RoomDevices::Thermometer(thermo2),
    ];
    let room2 = Room::new(String::from("Room2"), devices2);

    let rooms = HashMap::from([
        (room1.show_description().to_string(), room1),
        (room2.show_description().to_string(), room2),
    ]);
    let house = SmartHouse {
        description: String::from("House"),
        rooms,
    };
    house.rooms_info();
}

#[test]
fn test_reports() {
    let socket1 = Socket::build_socket(String::from("Socket1"));
    socket1.info();
    let thermo1 = Thermometer::build_thermometer(String::from("Thermometer1"));
    thermo1.info();
    let devices1: Vec<RoomDevices> = vec![
        RoomDevices::Socket(socket1),
        RoomDevices::Thermometer(thermo1),
    ];
    let room1 = Room::new(String::from("Room1"), devices1);
    room1.info();
    let house = SmartHouse {
        description: String::from("House"),
        rooms: HashMap::from([(room1.show_description().to_string(), room1)]),
    };
    house.rooms_info();
}
