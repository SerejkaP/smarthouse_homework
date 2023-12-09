use std::{
    io::{Read, Write},
    net::TcpListener,
};

use super::tcp_client::{Command, Response};

pub fn handle_tcp_connection() {
    let mut args = std::env::args();
    args.next().unwrap();

    let listener = TcpListener::bind("127.0.0.1:3000").expect("can't bind tcp listener");
    println!("TcpServer Listening on 127.0.0.1:3000");
    let mut smart_socket = TcpSmartSocket::default();

    while let Some(connection) = listener.incoming().next() {
        let mut stream = match connection {
            Ok(conn) => conn,
            Err(err) => {
                println!("can't receive connection: {err}");
                continue;
            }
        };

        let peer = stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "unknown".into());
        println!("Peer '{peer}' connected");

        let mut in_buffer = [0u8];
        while stream.read_exact(&mut in_buffer).is_ok() {
            let response = smart_socket.process_command(in_buffer[0].into());
            let response_buf: [u8; 5] = response.into();
            if stream.write_all(&response_buf).is_err() {
                break;
            };
        }

        println!("Connection with {peer} lost. Waiting for new connections...");
    }
}

#[derive(Default)]
struct TcpSmartSocket {
    active: bool,
}

impl TcpSmartSocket {
    fn process_command(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::TurnOn => {
                self.active = true;
                Response::Ok
            }
            Command::TurnOff => {
                self.active = false;
                Response::Ok
            }
            Command::GetStatus => {
                if self.active {
                    Response::On
                } else {
                    Response::Off
                }
            }
            Command::GetPower => {
                if self.active {
                    Response::Power(220.5)
                } else {
                    Response::Power(0.0)
                }
            }
            Command::Unknown => {
                println!("Unknown command received");
                Response::Unknown
            }
        }
    }
}
