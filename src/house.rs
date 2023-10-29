use super::room::Room;
use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Debug, Clone)]
pub struct SmartHouse {
    pub description: String,
    pub rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new(description: String) -> Self {
        Self {
            description,
            rooms: HashMap::new(),
        }
    }

    pub fn rooms_info(&self) -> Vec<&str> {
        self.rooms.iter().map(|r| r.1.show_description()).collect()
    }

    pub fn add_room(&mut self) {
        io::stdout().write_all(b"Write room name\n").unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_owned();
        if self.rooms.contains_key(buffer.as_str()) {
            io::stdout()
                .write_all(b"Room with this name already exists\n")
                .unwrap();
            return;
        }
        let room = Room::new(buffer, Vec::new());
        self.rooms.insert(room.show_description().to_string(), room);
    }

    pub fn remove_room(&mut self) {
        io::stdout()
            .write_all(b"Write room name to remove\n")
            .unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_owned();
        self.rooms.remove(buffer.as_str());
    }

    pub fn info(&self) -> String {
        let mut info = "House name: ".to_string() + &self.description + "\n";
        for room in &self.rooms {
            let room_name = "Room name: ".to_string() + room.0 + "\n";
            let devices_info = "\t".to_string() + room.1.info().join("\n\t").as_str();
            info += &(room_name + &devices_info + "\n");
        }
        info
    }

    pub fn devices_info(&self, room: &str) -> Result<String, GetRoomError> {
        let room = &self.rooms.get(room);
        match room {
            None => Err(GetRoomError {
                reason: "Room can not be found!\n".to_string(),
            }),
            Some(room) => {
                let devices = &room.info();
                Ok(devices.join("\n"))
            }
        }
    }

    pub fn add_device(&mut self, room: &str) {
        let room = self.rooms.get_mut(room);
        match room {
            None => io::stdout().write_all(b"Room can not be found!\n").unwrap(),
            Some(room) => {
                room.add_device();
            }
        }
    }

    pub fn remove_device(&mut self, room: &str) {
        let room = self.rooms.get_mut(room);
        match room {
            None => io::stdout().write_all(b"Room can not be found!\n").unwrap(),
            Some(room) => {
                room.remove_device();
            }
        }
    }
}

#[derive(Debug)]
pub struct GetRoomError {
    pub reason: String,
}
