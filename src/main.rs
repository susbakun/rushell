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
mod shell;
use shell::*;

use anyhow::{Result, anyhow};
use is_executable::IsExecutable;
use rustyline::{
    CompletionType, Config, Editor, error::ReadlineError::Interrupted, history::DefaultHistory,
};

fn main() -> Result<()> {
    let mut shell = Shell::new();
    shell.run()
}
