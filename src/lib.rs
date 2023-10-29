mod house;
mod room;
mod room_devices;
mod socket;
mod thermometer;

pub mod prelude {
    pub use crate::house::*;
    pub use crate::room::*;
    pub use crate::room_devices::*;
    pub use crate::socket::*;
    pub use crate::thermometer::*;
}
