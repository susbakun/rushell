use super::*;

use rustyline::Helper;
use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;

pub struct ShellHepler;

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

        let st = line.get(start..pos).unwrap_or_default();

        let candidates = KNOWN_COMMANDS
            .iter()
            .filter(|item| item.starts_with(st))
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
