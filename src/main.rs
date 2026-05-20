#[allow(unused_imports)]
use std::io::{self, Write};
use std::{fs, path::Path};

use is_executable::IsExecutable;

fn main() {
    let known_commands = ["type", "exit", "echo"];
    let paths = std::env::var("PATH").unwrap();
    let paths = paths.split(":").collect::<Vec<&str>>();

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
                println!("{remainder} is a shell builtin");
            } else {
                let mut found = false;
                'outer: for path in &paths {
                    let dir = fs::read_dir(path).unwrap();
                    for file in dir {
                        let file = file.unwrap();
                        let file_name = file.file_name().into_string().unwrap();
                        let path = file.path();
                        let file_path = Path::new(&path);

                        if file_path.is_executable() && file_name == remainder {
                            println!("{remainder} is {}", file_path.to_str().unwrap());
                            found = true;
                            break 'outer;
                        }
                    }
                }

                if !found {
                    println!("{remainder}: not found");
                }
            }
        } else {
            println!("{command}: command not found");
        }
    }
}
