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
mod remainder;
use remainder::*;

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

        if buffer.starts_with("'") || buffer.starts_with("'") {
            let (command, remainder) = parse_buffer(&buffer);
            process_command(&command, &remainder, &paths)?;
        } else {
            let (command, remainder) = &buffer.split_once(" ").unwrap_or((buffer, ""));
            process_command(command, remainder, &paths)?;
        }
    }
}

fn parse_buffer(buffer: &str) -> (String, String) {
    let mut command = String::new();
    let mut sep = 0;

    let (mut single_quote, mut double_quote) = (false, false);

    if buffer.starts_with("'") {
        single_quote = true;
    } else if buffer.starts_with("\"") {
        double_quote = true;
    }

    for (ind, ch) in buffer.char_indices().skip(1) {
        if ch == '\'' && single_quote {
            sep = ind;
            break;
        } else if ch == '\"' && double_quote {
            sep = ind;
            break;
        } else {
            command.push(ch);
        }
    }

    println!("{command}");

    let remainder = buffer.get(sep + 2..).unwrap().to_string();

    (command, remainder)
}
