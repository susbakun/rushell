#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let known_commands = ["type", "exit", "echo"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();

        std::io::stdin().read_line(&mut buffer).unwrap();

        let buffer = buffer.trim();
        let line = &buffer.split(" ").collect::<Vec<&str>>();
        let (command, remainder) = line.split_at(1);
        let command = command[0];
        let remainder = remainder.join(" ");

        if command == "exit" {
            return;
        } else if command == "echo" {
            println!("{remainder}")
        } else if command == "type" {
            if known_commands.contains(&&remainder[..]) {
                println!("{command}: is a shell builtin");
            } else {
                println!("{command}: not found");
            }
        } else {
            println!("{command}: command not found");
        }
    }
}
