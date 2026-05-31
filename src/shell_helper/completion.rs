use std::io::Write;

use rustyline::completion::Completer;

use crate::constants::KNOWN_COMMANDS;

use super::ShellHepler;

impl Completer for ShellHepler {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let prefix = current_word(line, pos);
        let candidates = matching_commands(&self.exe_commands, prefix);

        let extended = longest_common_prefix(prefix, &candidates);
        if prefix != extended {
            return Ok((0, vec![extended]));
        }

        match candidates.len() {
            0 | 1 => Ok((0, candidates)),
            _ => self.complete_ambiguous(line, prefix, &candidates),
        }
    }
}

impl ShellHepler {
    fn complete_ambiguous(
        &self,
        line: &str,
        prefix: &str,
        candidates: &[String],
    ) -> rustyline::Result<(usize, Vec<String>)> {
        match self.tab_press_count(prefix) {
            1 => ring_bell()?,
            _ => print_candidates(line, candidates)?,
        }
        Ok((0, vec![]))
    }
}

fn current_word(line: &str, pos: usize) -> &str {
    let mut start = 0;
    for (index, ch) in line.char_indices().rev() {
        if ch == ' ' {
            start = index + 1;
        }
    }
    line.get(start..pos).unwrap_or_default()
}

fn matching_commands(exe_commands: &[String], prefix: &str) -> Vec<String> {
    let mut candidates: Vec<String> = exe_commands
        .iter()
        .filter(|command| command.starts_with(prefix))
        .map(|command| format!("{command} "))
        .collect();

    for command in KNOWN_COMMANDS {
        if command.starts_with(prefix) {
            candidates.push(format!("{command} "));
        }
    }

    candidates.sort();
    candidates.dedup();
    candidates
}

fn longest_common_prefix(prefix: &str, candidates: &[String]) -> String {
    let mut extended = prefix.to_string();
    let Some(first) = candidates.first() else {
        return extended;
    };

    for ch in first.chars().skip(prefix.chars().count()) {
        extended.push(ch);
        if candidates.iter().any(|candidate| !candidate.starts_with(&extended)) {
            extended.pop();
            break;
        }
    }

    extended
}

fn ring_bell() -> rustyline::Result<()> {
    print!("\x07");
    std::io::stdout().flush()?;
    Ok(())
}

fn print_candidates(line: &str, candidates: &[String]) -> rustyline::Result<()> {
    println!();
    println!("{}", candidates.join("  "));
    print!("$ {line}");
    std::io::stdout().flush()?;
    Ok(())
}
