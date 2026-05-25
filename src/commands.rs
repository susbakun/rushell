use super::*;

pub fn process_command(
    command: &str,
    remainder: &str,
    args: &[String],
    paths: &Vec<&str>,
) -> Result<()> {
    if command == "exit" {
        handle_exit_command();
    } else if command == "echo" {
        handle_echo_command(remainder, args)?;
    } else if command == "type" {
        handle_type_command(remainder, &paths, args)?;
    } else {
        handle_executable_command(args, command, &paths)?;
    }

    Ok(())
}

fn handle_exit_command() {
    std::process::exit(0);
}

fn handle_echo_command(remainder: &str, args: &[String]) -> Result<usize> {
    let output = format!("{remainder}\n");
    process_output(&output, args, false)
}

fn handle_type_command(remainder: &str, paths: &Vec<&str>, args: &[String]) -> Result<usize> {
    if KNOWN_COMMANDS.contains(&&remainder[..]) {
        let output = format!("{remainder} is a shell builtin");
        process_output(&output, args, false)?;
    } else {
        let (file_path, found) = find_exe(paths, &remainder)?;
        if found {
            let file_path = file_path.unwrap();

            let output = format!("{remainder} is {}", file_path.to_str().unwrap());
            process_output(&output, args, false)?;
        } else {
            let output = format!("{remainder}: not found");
            process_output(&output, args, true)?;
        }
    }

    Ok(0)
}

fn handle_executable_command(args: &[String], command: &str, paths: &Vec<&str>) -> Result<usize> {
    let (_, found) = find_exe(paths, command)?;
    let exec_args = args_without_redirect(args);

    if found {
        let command_output = Command::new(command)
            .args(exec_args)
            .output()
            .expect("failed to execute the command");

        let stderr = String::from_utf8_lossy(&command_output.stderr).to_string();
        if !stderr.is_empty() {
            process_output(&stderr, args, true)?;
        }

        let stdout = String::from_utf8_lossy(&command_output.stdout);
        if !stdout.is_empty() {
            process_output(&stdout.to_string(), args, false)?;
        }
    } else {
        let output = format!("{command}: not found");
        process_output(&output, args, true)?;
    }

    Ok(0)
}
