use std::io;

use smarthouse_homework::prelude::*;

fn main() {
    println!("Select server type: 1) UDP, 2) TCP");

    let mut buff = String::new();
    io::stdin().read_line(&mut buff).unwrap();
    if buff.trim() == "1" {
        handle_udp_connection();
    } else if buff.trim() == "2" {
        handle_tcp_connection();
    }

    // // Инициализация устройств
    // let socket1 = Socket::build_socket(String::from("Socket1"));
    // let thermo1 = Thermometer::build_thermometer(String::from("Thermometer1"));

    // // Инициализация дома
    // let devices1: Vec<RoomDevices> = vec![
    //     RoomDevices::Socket(socket1),
    //     RoomDevices::Thermometer(thermo1),
    // ];
    // let room1 = Room::new(String::from("Room1"), devices1);
    // let mut house = SmartHouse {
    //     description: String::from("House"),
    //     rooms: HashMap::from([(room1.show_description().to_string(), room1)]),
    // };

    // // Добавляем
    // house.add_room();
    // house.add_room();
    // house.add_room();
    // house.rooms_info();
    // let rooms = house.rooms_info();
    // println!("Now rooms: {:#?}", rooms);
    // io::stdout()
    //     .write_all(b"Write room name to add device in it\n")
    //     .unwrap();
    // let mut buff = String::new();
    // io::stdin().read_line(&mut buff).unwrap();
    // let room_description = buff.trim().to_string();
    // house.add_device(room_description.as_str());
    // let rooms_info = house.rooms_info();
    // let house_info = house.info();
    // println!("Rooms info: {:#?}", rooms_info);
    // io::stdout().write_all(house_info.as_bytes()).unwrap();

    // // Удаляем
    // house.remove_room();
    // let rooms_info = house.rooms_info();
    // println!("Now rooms: {:#?}", rooms_info);
    // io::stdout()
    //     .write_all(b"Write room name to remove device in it\n")
    //     .unwrap();
    // let mut buff = String::new();
    // io::stdin().read_line(&mut buff).unwrap();
    // let room_description = buff.trim().to_string();
    // house.remove_device(room_description.as_str());
    // let rooms_info = house.rooms_info();
    // let house_info = house.info();
    // println!("Rooms info: {:#?}", rooms_info);
    // io::stdout().write_all(house_info.as_bytes()).unwrap();
}
