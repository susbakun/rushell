use super::*;

use rustyline::Helper;
use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;

pub struct ShellHepler {
    exe_commands: Vec<String>,
}

impl ShellHepler {
    pub fn new(exe_commands: Vec<String>) -> Self {
        Self { exe_commands }
    }
}

impl Completer for ShellHepler {
    type Candidate = String;

    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut start = 0;

        for (ind, ch) in line.char_indices().rev() {
            if ch == ' ' {
                start = ind
            }
        }

        let mut str_commands = self
            .exe_commands
            .iter()
            .map(|command| command.as_str())
            .collect::<Vec<&str>>();
        str_commands.extend(KNOWN_COMMANDS);

        let patt = line.get(start..pos).unwrap_or_default();

        let candidates = str_commands
            .iter()
            .filter(|item| item.starts_with(patt))
            .map(|item| format!("{item} "))
            .collect::<Vec<String>>();
        // println!("{candidates:?}");
        Ok((0, candidates))
    }
}

impl Hinter for ShellHepler {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Highlighter for ShellHepler {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        line.into()
    }
}

impl Validator for ShellHepler {
    fn validate(
        &self,
        _ctx: &mut rustyline::validate::ValidationContext,
    ) -> rustyline::Result<rustyline::validate::ValidationResult> {
        Ok(rustyline::validate::ValidationResult::Valid(None))
    }
}

impl Helper for ShellHepler {}
