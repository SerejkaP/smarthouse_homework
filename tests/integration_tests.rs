use homework1::prelude::*;

#[test]
fn test_house() {
    // Инициализация устройств
    let socket1 = Socket::build_socket(String::from("Socket1"));
    let socket2 = Socket::build_socket(String::from("Socket2"));
    let _thermo = Thermometer::build_thermometer();

    // Инициализация дома
    let devices1 = vec![socket1.show_description().to_string()];
    let room1 = Room::new(String::from("Room1"), devices1);
    let devices2 = vec![socket2.show_description().to_string()];
    let room2 = Room::new(String::from("Room2"), devices2);
    let rooms = vec![&room1, &room2];
    let house = SmartHouse { _rooms: rooms };
    house.get_rooms();
    house.devices("Room1");
    house.devices("Room2");
}

#[test]
fn test_reports() {
    let socket1 = Socket::build_socket(String::from("Socket1"));
    let socket2 = Socket::build_socket(String::from("Socket2"));
    let thermo = Thermometer::build_thermometer();

    let rooms = Vec::new();
    let house = SmartHouse { _rooms: rooms };

    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let _report1 = house.create_report(&info_provider_1.info());

    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let _report2 = house.create_report(&info_provider_2.info());
}
