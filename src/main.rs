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
mod file;
use file::*;

use anyhow::{Result, anyhow};
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
        if buffer.is_empty() {
            continue;
        }

        let tokens = parse_input(buffer);
        if tokens.is_empty() {
            continue;
        }
        let command = &tokens[0];
        let args = tokens.get(1..).unwrap_or_default();

        let remainder = &args.join(" ");
        let remainder = process_remainder(remainder);

        process_command(command, remainder, args, &paths)?;

        println!("")
    }
}
