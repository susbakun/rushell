use std::{fmt::format, fs, io::Write, path::PathBuf, process::Command};

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
        let (start, command, prefix) = current_word(line, pos);

        if start > 0 {
            // checking complete commands
            if let Some(path) = self.complete_commands.get(command) {
                return self.complete_registered_command(line, start, pos, path, prefix, command);
            }
            // maybe it's a path
            return self.complete_path(start, prefix, line);
        }

        // otherwise we look for the exe commands candidates
        let candidates = matching_commands(&self.exe_commands, prefix);

        let extended = longest_common_prefix(prefix, &candidates);
        if prefix != extended {
            return Ok((start, vec![extended]));
        }

        match candidates.len() {
            0 | 1 => Ok((start, candidates)),
            _ => self.complete_ambiguous(start, line, prefix, &candidates),
        }
    }
}

impl ShellHepler {
    fn complete_registered_command(
        &self,
        line: &str,
        start: usize,
        pos: usize,
        path: &String,
        prefix: &str,
        command: &str,
    ) -> rustyline::Result<(usize, Vec<String>)> {
        let args = line.split_whitespace().collect::<Vec<&str>>();
        let previous = if args.len() >= 3 {
            args[args.len() - 2]
        } else {
            ""
        };

        // setting the env variables necessary
        // for the completion script
        unsafe {
            std::env::set_var("COMP_LINE", line);
            std::env::set_var("COMP_POINT", pos.to_string());
        }

        let output = Command::new(path)
            .arg(command)
            .arg(prefix)
            .arg(previous)
            .output()?;
        let stdout = String::from_utf8_lossy(output.stdout.as_slice());

        let candidates: Vec<String> = stdout
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|candidate| format!("{candidate} "))
            .collect();

        match candidates.len() {
            0 | 1 => Ok((start, candidates)),
            _ => self.complete_ambiguous(start, line, prefix, &candidates),
        }
    }
    fn complete_path(
        &self,
        start: usize,
        prefix: &str,
        line: &str,
    ) -> rustyline::Result<(usize, Vec<String>)> {
        let candidates = matching_paths(prefix);

        let extended = longest_common_prefix(prefix, &candidates);
        if prefix != extended {
            return Ok((start, vec![extended]));
        }

        match candidates.len() {
            0 | 1 => Ok((start, candidates)),
            _ => self.complete_ambiguous(start, line, prefix, &candidates),
        }
    }

    fn complete_ambiguous(
        &self,
        start: usize,
        line: &str,
        prefix: &str,
        candidates: &[String],
    ) -> rustyline::Result<(usize, Vec<String>)> {
        match self.tab_press_count(prefix) {
            1 => ring_bell()?,
            _ => print_candidates(line, candidates)?,
        }
        Ok((start, vec![]))
    }
}

fn current_word(line: &str, pos: usize) -> (usize, &str, &str) {
    let mut start = 0;
    for (index, ch) in line.char_indices().rev() {
        if ch == ' ' {
            start = index + 1;
            break;
        } else if index == 0 {
            start = index;
        }
    }

    let command = line.split(" ").next().unwrap_or_default();

    (start, command, line.get(start..pos).unwrap_or_default())
}

fn matching_paths(prefix: &str) -> Vec<String> {
    let (dir, name_prefix, base) = path_parts(prefix);
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    let mut candidates = entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.starts_with(name_prefix) {
                return None;
            }

            let suffix = if entry.path().is_dir() { "/" } else { " " };
            Some(format!("{base}{name}{suffix}"))
        })
        .collect::<Vec<String>>();

    candidates.sort();
    candidates.dedup();
    candidates
}

fn path_parts(prefix: &str) -> (PathBuf, &str, &str) {
    if prefix.is_empty() {
        return (PathBuf::from("."), "", "");
    }

    if prefix.ends_with('/') {
        return (PathBuf::from(prefix), "", prefix);
    }

    if let Some(index) = prefix.rfind('/') {
        let dir = PathBuf::from(&prefix[..index + 1]);
        let name_prefix = &prefix[index + 1..];
        let base = &prefix[..index + 1];
        return (dir, name_prefix, base);
    }

    (PathBuf::from("."), prefix, "")
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
        if candidates
            .iter()
            .any(|candidate| !candidate.starts_with(&extended))
        {
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
