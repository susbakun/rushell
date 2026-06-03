#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use is_executable::IsExecutable;

use rustyline::error::ReadlineError::Interrupted;

use rustyline::{CompletionType, Config};

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
mod shell;
use shell::*;
mod job;
use job::*;
mod types;
use types::*;

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let mut shell = Shell::new()?;
    shell.run()
}
