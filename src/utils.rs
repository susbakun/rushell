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

pub fn args_without_redirect(args: &[String]) -> Vec<String> {
    let mut new_args = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if STDOUT_REDIRECT_OPS.contains(&args[i].as_str())
            || STDERROR_REDIRECT_OPS.contains(&args[i].as_str())
            || STDOUT_APPEND_OPS.contains(&args[i].as_str())
            || STDERROR_APPEND_OPS.contains(&args[i].as_str())
        {
            break;
        }
        new_args.push(args[i].clone());
        i += 1;
    }
    new_args
}

pub fn find_exe(paths: &Vec<String>, command: &str) -> Result<(Option<PathBuf>, bool)> {
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

pub fn find_command_names_on_path(paths: &Vec<String>) -> Result<Vec<String>> {
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
