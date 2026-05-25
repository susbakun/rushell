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

pub fn process_remainder(remainder: &str) -> &str {
    for op in ["1>"] {
        if let Some(idx) = remainder.find(op) {
            return remainder[..idx].trim_end();
        }
    }

    remainder
}

pub fn find_exe(paths: &Vec<&str>, command: &str) -> Result<(Option<PathBuf>, bool)> {
    let mut found = false;
    for path in paths {
        let dir = fs::read_dir(path)?;
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

    Ok((None, found))
}

pub fn should_redirect(args: &[String]) -> bool {
    let redirect_operator = String::from(">");
    let redirect_operator2 = String::from("1>");

    args.contains(&redirect_operator) || args.contains(&redirect_operator2)
}

pub fn process_output(output: &String, args: &[String], new_line: bool) -> Result<usize> {
    if should_redirect(args) {
        write_to_file(output, args)
    } else {
        if new_line {
            println!("{output}");
        } else {
            print!("{output}");
            io::stdout().flush()?;
        }
        Ok(0)
    }
}
