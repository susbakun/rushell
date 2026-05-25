use anyhow::Ok;

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
        {
            break;
        }
        new_args.push(args[i].clone());
        i += 1;
    }
    new_args
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

pub fn process_output(output: &String, args: &[String], is_stderror: bool) -> Result<usize> {
    if should_redirect(args, is_stderror) {
        let file = find_file(args)?;

        write_to_file(output, file)?;
        return Ok(0);
    }
    if !output.is_empty() {
        print!("{output}");
        if !output.ends_with('\n') {
            println!();
        }
    }
    io::stdout().flush()?;
    Ok(0)
}

pub fn should_redirect(args: &[String], is_stderror: bool) -> bool {
    if is_stderror {
        return has_err_redirect(args); // Only redirect stderr if 2> is present
    } else {
        return has_std_redirect(args); // Only redirect stdout if > is present
    }
}

pub fn has_std_redirect(args: &[String]) -> bool {
    args.iter()
        .any(|arg| STDOUT_REDIRECT_OPS.contains(&arg.as_str()))
}

pub fn has_err_redirect(args: &[String]) -> bool {
    args.iter()
        .any(|arg| STDERROR_REDIRECT_OPS.contains(&arg.as_str()))
}
