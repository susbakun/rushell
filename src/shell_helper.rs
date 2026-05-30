use std::cell::RefCell;

use super::*;

use rustyline::Helper;
use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;

pub struct ShellHepler {
    exe_commands: Vec<String>,
    last_prefix: RefCell<Option<String>>,
    tab_count: RefCell<usize>,
}

impl ShellHepler {
    pub fn new(exe_commands: Vec<String>) -> Self {
        Self {
            exe_commands,
            last_prefix: RefCell::new(None),
            tab_count: RefCell::new(0),
        }
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
                start = ind + 1
            }
        }

        let mut str_commands = self
            .exe_commands
            .iter()
            .map(|command| command.as_str())
            .collect::<Vec<&str>>();
        str_commands.extend(KNOWN_COMMANDS);

        let patt = line.get(start..pos).unwrap_or_default();

        let mut candidates = str_commands
            .iter()
            .filter(|item| item.starts_with(patt))
            .map(|item| format!("{item} "))
            .collect::<Vec<String>>();

        candidates.sort();
        // removing duplicates
        candidates.dedup();

        if candidates.len() > 1 {
            let prefix = patt.to_string();
            let same_prefix = self.last_prefix.borrow().as_ref() == Some(&prefix);

            if same_prefix {
                *self.tab_count.borrow_mut() += 1;
            } else {
                *self.tab_count.borrow_mut() = 1;
                *self.last_prefix.borrow_mut() = Some(prefix);
            }

            // first tab
            if *self.tab_count.borrow() == 1 {
                print!("\x07");
                std::io::stdout().flush()?;
                return Ok((0, vec![]));
            }

            // second tab
            let joined_candidates = candidates.join("  ");
            println!();
            println!("{joined_candidates}");
            print!("$ {line}");
            std::io::stdout().flush()?;
            Ok((0, vec![]))
        } else {
            Ok((0, candidates))
        }
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
