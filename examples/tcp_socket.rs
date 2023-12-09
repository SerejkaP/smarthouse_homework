use std::io;

use smarthouse_homework::prelude::{Command, TcpSocket};

fn main() {
    let mut client = TcpSocket::new("127.0.0.1:3000").unwrap();

    loop {
        show_menu();
        let input = read_input();

        let response = match input {
            Some(command) => client.run_command(command).unwrap(),
            None => {
                println!("Connection closed");
                break;
            }
        };

        println!("------------------");
        println!("Response: {response}");
    }
}

fn show_menu() {
    println!();
    println!("------------------");
    println!("Select action:");
    println!("1) turn off");
    println!("2) turn on");
    println!("3) status");
    println!("4) power");
    println!("_) exit");
}

fn read_input() -> Option<Command> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cmd = match input.trim() {
        "1" => Command::TurnOff,
        "2" => Command::TurnOn,
        "3" => Command::GetStatus,
        "4" => Command::GetPower,
        _ => return None,
    };

    Some(cmd)
}
