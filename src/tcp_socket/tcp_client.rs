use std::{
    error::Error,
    fmt,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

pub enum Command {
    TurnOn,
    TurnOff,
    GetPower,
    GetStatus,
    Unknown,
}

impl From<u8> for Command {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::TurnOff,
            1 => Self::TurnOn,
            2 => Self::GetPower,
            3 => Self::GetStatus,
            _ => Self::Unknown,
        }
    }
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::TurnOff => 0,
            Command::TurnOn => 1,
            Command::GetPower => 2,
            Command::GetStatus => 3,
            Command::Unknown => 255,
        }
    }
}

pub enum Response {
    Ok,
    On,
    Off,
    Power(f32),
    Unknown,
}

impl From<[u8; 5]> for Response {
    fn from(bytes: [u8; 5]) -> Self {
        match bytes {
            [0, ..] => Self::Ok,
            [1, ..] => Self::On,
            [2, ..] => Self::Off,
            [3, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[1..]);
                Self::Power(f32::from_be_bytes(buf))
            }
            _ => Self::Unknown,
        }
    }
}

impl From<Response> for [u8; 5] {
    fn from(resp: Response) -> Self {
        let mut buffer = [0u8; 5];
        match resp {
            Response::Ok => {}
            Response::On => buffer[0] = 1,
            Response::Off => buffer[0] = 2,
            Response::Power(pwr) => {
                buffer[0] = 3;
                buffer[1..].copy_from_slice(&pwr.to_be_bytes())
            }
            Response::Unknown => buffer[0] = 255,
        };
        buffer
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Ok => write!(f, "Ok"),
            Response::On => write!(f, "Socket is On"),
            Response::Off => write!(f, "Socket is Off"),
            Response::Power(power) => write!(f, "Power: {}", power),
            Response::Unknown => write!(f, "Unknown"),
        }
    }
}

pub struct TcpSocket {
    stream: TcpStream,
}

impl TcpSocket {
    pub fn new(server_address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(server_address)?;
        Ok(Self { stream })
    }

    pub fn run_command(&mut self, command: Command) -> Result<Response, Box<dyn Error>> {
        self.stream.write_all(&[command.into()])?;
        let mut buffer = [0u8; 5];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer.into())
    }
}
