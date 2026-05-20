#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
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
        } else {
            println!("{command}: command not found");
        }
    }
}
