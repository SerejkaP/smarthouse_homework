use super::room::Room;

pub struct SmartHouse<'a> {
    pub _rooms: Vec<&'a Room>,
}

impl SmartHouse<'_> {
    pub fn get_rooms(&self) -> Vec<&str> {
        let mut rooms = Vec::new();
        for room in &self._rooms {
            rooms.push(room.show_description())
        }
        rooms
    }

    pub fn devices(&self, room: &str) -> Result<&Vec<String>, GetRoomError> {
        let room = &self._rooms.iter().find(|r| r.description == room);
        match room {
            None => Err(GetRoomError {
                reason: "Room can not be found!".to_string(),
            }),
            Some(room) => Ok(room.show_devices()),
        }
    }

    pub fn create_report(
        &self,
        info: &str, /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
    ) -> String {
        info.to_string()
    }
}

#[derive(Debug)]
pub struct GetRoomError {
    pub reason: String,
}
