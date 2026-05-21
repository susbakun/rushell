#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

mod commands;
use commands::*;
mod utils;
use utils::*;

use anyhow::Result;
use is_executable::IsExecutable;

const KNOWN_COMMANDS: &[&str] = &["type", "exit", "echo"];

fn main() -> Result<()> {
    let paths = std::env::var("PATH").unwrap();
    let paths = paths.split(":").collect::<Vec<&str>>();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();

        std::io::stdin().read_line(&mut buffer).unwrap();

        let buffer = buffer.trim();
        let (command, remainder) = &buffer.split_once(" ").unwrap();
        let remainder = process_remainder(remainder);

        process_command(command, &remainder, &paths)?;
    }
}

fn process_remainder(remainder: &str) -> String {
    let mut formated = String::new();
    let mut single_quote = false;
    let mut space_count = 0;

    for ch in remainder.chars() {
        if ch == '\'' {
            single_quote = !single_quote;
        } else if single_quote {
            formated.push(ch);
        } else if ch == ' ' {
            space_count += 1;
            if space_count < 2 {
                formated.push(ch);
            }
        } else {
            formated.push(ch);
            space_count = 0;
        }
    }

    formated
}
