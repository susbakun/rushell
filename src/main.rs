#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use is_executable::IsExecutable;

use rustyline::error::ReadlineError::Interrupted;
use rustyline::history::DefaultHistory;
use rustyline::{CompletionType, Config, Editor};

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
mod pipeline;
use pipeline::*;
mod job;
use job::*;

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let mut shell = Shell::new()?;
    shell.run()
}
