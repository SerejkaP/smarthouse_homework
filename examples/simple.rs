use smarthouse_homework::prelude::*;

fn main() {
    // Инициализация устройств
    let socket1 = Socket::build_socket(String::from("Socket1"));
    let socket2 = Socket::build_socket(String::from("Socket2"));
    let thermo = Thermometer::build_thermometer();

    // Инициализация дома
    let devices1 = vec![socket1.show_description().to_string()];
    let room1 = Room::new(String::from("Room1"), devices1);
    let devices2 = vec![socket2.show_description().to_string()];
    let room2 = Room::new(String::from("Room2"), devices2);
    let rooms = vec![&room1, &room2];
    let house = SmartHouse { _rooms: rooms };

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1.info());

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2.info());

    let rooms_report = house.get_rooms();
    let room1_devices = house.devices(rooms_report.first().unwrap());

    println!("Rooms: {:#?}", rooms_report);
    println!("Room1 devices: {:#?}", room1_devices);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
