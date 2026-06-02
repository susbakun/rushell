use super::*;

pub fn parse_input(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let (mut is_single_quote, mut is_double_quote) = (false, false);
    let mut backslash_seen = false;

    for ch in input.chars() {
        if backslash_seen {
            current.push(ch);
            backslash_seen = false;
            continue;
        }

        match ch {
            '\'' | '"' => {
                if is_double_quote {
                    if ch == '"' {
                        is_double_quote = !is_double_quote;
                    } else {
                        current.push(ch);
                    }
                } else if is_single_quote {
                    if ch == '\'' {
                        is_single_quote = !is_single_quote;
                    } else {
                        current.push(ch);
                    }
                } else {
                    if ch == '\'' {
                        is_single_quote = !is_single_quote;
                    } else {
                        is_double_quote = !is_double_quote;
                    }
                }
            }
            '\\' => {
                if is_single_quote {
                    current.push(ch);
                } else {
                    backslash_seen = true;
                }
            }
            ' ' if !is_single_quote && !is_double_quote => {
                if !current.is_empty() {
                    args.push(current);
                    current = String::new();
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}

pub fn parse_args(args: &[String]) -> Vec<String> {
    segment_args(args)
}

pub fn segment_args(args: &[String]) -> Vec<String> {
    let mut new_args = Vec::new();
    for arg in args {
        if STDOUT_REDIRECT_OPS.contains(&arg.as_str())
            || STDERROR_REDIRECT_OPS.contains(&arg.as_str())
            || STDOUT_APPEND_OPS.contains(&arg.as_str())
            || STDERROR_APPEND_OPS.contains(&arg.as_str())
            || arg == "&"
            || arg == "|"
        {
            break;
        }
        new_args.push(arg.clone());
    }
    new_args
}

pub fn split_pipeline(tokens: &Vec<String>) -> Result<Vec<PipelineParts>> {
    let mut parts = vec![];

    let iter = tokens.split(|item| item == "|");

    for item in iter {
        if item.is_empty() {
            continue;
        }

        let command = item.get(0).unwrap().to_string();
        let args = item.get(1..).unwrap_or_default().to_vec();

        let part = PipelineParts { args, command };
        parts.push(part);
    }

    Ok(parts)
}

pub fn find_exe(paths: &[String], command: &str) -> Result<(Option<PathBuf>, bool)> {
    let mut found = false;
    for path in paths {
        if path.is_empty() {
            continue;
        }

        if let Ok(dir) = fs::read_dir(path) {
            for file in dir {
                let file = file?;
                let file_name = file.file_name();

                let path = file.path();
                let file_path = Path::new(&path);

                if file_path.is_executable() && file_name == command {
                    found = true;
                    let file_path = file_path.to_owned();
                    return Ok((Some(file_path), found));
                }
            }
        }
    }

    Ok((None, found))
}

pub fn find_command_names_on_path(paths: &[String]) -> Result<Vec<String>> {
    let mut commands: Vec<String> = Vec::new();

    for path in paths {
        if path.is_empty() {
            continue;
        }

        if let Ok(dir) = fs::read_dir(path) {
            for file in dir {
                let file = file?;
                let entry = file.file_name();
                let name = entry.to_string_lossy().into();
                let file_path = file.path();

                let file_path = Path::new(&file_path);
                if file_path.is_executable() {
                    commands.push(name);
                }
            }
        }
    }

    Ok(commands)
}

pub fn is_piped(args: &[String]) -> bool {
    args.contains(&"|".to_string())
}

pub fn is_builtin_command(command: &str) -> bool {
    KNOWN_COMMANDS.contains(&command)
}

pub fn format_echo_output(remainder: &str) -> String {
    format!("{remainder}\n")
}

pub fn format_type_output(paths: &[String], remainder: &str) -> Result<String> {
    if is_builtin_command(remainder) {
        Ok(format!("{remainder} is a shell builtin\n"))
    } else {
        let (file_path, found) = find_exe(paths, remainder)?;
        if found {
            Ok(format!(
                "{remainder} is {}\n",
                file_path.unwrap().to_str().unwrap()
            ))
        } else {
            Ok(format!("{remainder}: not found\n"))
        }
    }
}
