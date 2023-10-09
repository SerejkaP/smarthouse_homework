mod house;
mod info_provider;
mod room;
mod socket;
mod thermometer;

pub mod prelude {
    pub use crate::house::*;
    pub use crate::info_provider::*;
    pub use crate::room::*;
    pub use crate::socket::*;
    pub use crate::thermometer::*;
}
