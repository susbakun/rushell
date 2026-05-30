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
mod constants;
use constants::*;
mod output;
use output::*;
mod shell_helper;
use shell_helper::*;

use anyhow::{Result, anyhow};
use is_executable::IsExecutable;
use rustyline::{Editor, error::ReadlineError::Interrupted, history::DefaultHistory};

const KNOWN_COMMANDS: &[&str] = &["type", "exit", "echo"];

fn main() -> Result<()> {
    let paths = std::env::var("PATH").unwrap();
    let paths = paths.split(":").collect::<Vec<&str>>();

    let helper = ShellHepler {};

    let mut rl = Editor::<ShellHepler, DefaultHistory>::new()?;
    rl.set_helper(Some(helper));

    loop {
        let readline = rl.readline("$ ");

        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }

                let tokens = parse_input(&line);
                if tokens.is_empty() {
                    continue;
                }
                let command = &tokens[0];
                let args = tokens.get(1..).unwrap_or_default();

                let remainder = args_without_redirect(args).join(" ");

                process_command(command, &remainder, args, &paths)?;
            }
            Err(Interrupted) => break Ok(()),
            Err(err) => break Err(anyhow!("{err}")),
        }
    }
}
