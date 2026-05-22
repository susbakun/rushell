use super::*;

pub fn process_command(command: &str, remainder: &str, paths: &Vec<&str>) -> Result<()> {
    if command == "exit" {
        handle_exit_command();
    } else if command == "echo" {
        handle_echo_command(remainder);
    } else if command == "type" {
        handle_type_command(remainder, &paths)?;
    } else {
        handle_executable_command(remainder, command, &paths)?;
    }

    Ok(())
}

fn handle_exit_command() {
    std::process::exit(0);
}

fn handle_echo_command(remainder: &str) {
    let remainder = process_remainder(remainder);

    println!("{remainder}")
}

fn handle_type_command(remainder: &str, paths: &Vec<&str>) -> Result<()> {
    let remainder = process_remainder(remainder);

    if KNOWN_COMMANDS.contains(&&remainder[..]) {
        println!("{remainder} is a shell builtin");
    } else {
        let (file_path, found) = find_exe(paths, &remainder)?;
        if found {
            let file_path = file_path.unwrap();

            println!("{remainder} is {}", file_path.to_str().unwrap());
        } else {
            println!("{remainder}: not found");
        }
    }

    Ok(())
}

fn handle_executable_command(remainder: &str, command: &str, paths: &Vec<&str>) -> Result<()> {
    let command = format_command(command);
    let (_, found) = find_exe(paths, &command)?;
    if found {
        let args = parse_args(remainder);
        let output = Command::new(command)
            .args(&args)
            .output()
            .expect("failed to execute the program");

        print!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("{command}: not found");
    }

    Ok(())
}

fn format_command(command: &str) -> String {
    let mut foramtted_command = String::new();
    let (mut is_single_quote, mut is_double_quote) = (false, false);

    for ch in command.chars() {
        if ch == '\'' || ch == '"' {
            if is_double_quote {
                if ch == '"' {
                    is_double_quote = !is_double_quote;
                } else {
                    foramtted_command.push(ch);
                }
            } else if is_single_quote {
                if ch == '\'' {
                    is_single_quote = !is_single_quote;
                } else {
                    foramtted_command.push(ch);
                }
            } else {
                if ch == '\'' {
                    is_single_quote = !is_single_quote;
                } else {
                    is_double_quote = !is_double_quote;
                }
            }
        } else {
            foramtted_command.push(ch);
        }
    }

    foramtted_command
}
