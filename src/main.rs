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
        let (command, remainder) = &buffer.split_once(" ").unwrap_or((buffer, ""));

        process_command(command, remainder, &paths)?;
    }
}
