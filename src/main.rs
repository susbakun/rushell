#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

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

        if command == "exit" {
            return;
        } else if command == "echo" {
            let remainder = remainder.join(" ");
            println!("{remainder}")
        } else if command == "type" {
            let remainder = remainder.join(" ");
            if known_commands.contains(&&remainder[..]) {
                println!("{remainder} is a shell builtin");
            } else {
                let (file_path, found) = find_exe(&paths, command);
                if found {
                    let file_path = file_path.unwrap();

                    println!("{command} is {}", file_path.to_str().unwrap());
                } else {
                    println!("{command}: not found");
                }
            }
        } else {
            let (_, found) = find_exe(&paths, command);
            if found {
                let output = Command::new(command)
                    .args(remainder)
                    .output()
                    .expect("failed to execute the program");

                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                println!("{command}: not found");
            }
        }
    }
}

fn find_exe(paths: &Vec<&str>, command: &str) -> (Option<PathBuf>, bool) {
    let mut found = false;
    for path in paths {
        let dir = fs::read_dir(path).unwrap();
        for file in dir {
            let file = file.unwrap();
            let file_name = file.file_name().into_string().unwrap();
            let path = file.path();
            let file_path = Path::new(&path);

            if file_path.is_executable() && file_name == command {
                found = true;
                let file_path = file_path.to_owned();
                return (Some(file_path), found);
            }
        }
    }

    (None, found)
}
