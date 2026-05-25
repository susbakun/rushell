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
    let output = remainder.to_string();
    process_output(&output, args, true)
}

fn handle_type_command(remainder: &str, paths: &Vec<&str>, args: &[String]) -> Result<usize> {
    if KNOWN_COMMANDS.contains(&&remainder[..]) {
        let output = format!("{remainder} is a shell builtin");
        process_output(&output, args, true)?;
    } else {
        let (file_path, found) = find_exe(paths, &remainder)?;
        if found {
            let file_path = file_path.unwrap();

            let output = format!("{remainder} is {}", file_path.to_str().unwrap());
            process_output(&output, args, true)?;
        } else {
            let output = format!("{remainder}: not found");
            process_output(&output, args, true)?;
        }
    }

    Ok(0)
}

fn handle_executable_command(args: &[String], command: &str, paths: &Vec<&str>) -> Result<usize> {
    let (_, found) = find_exe(paths, command)?;
    if found {
        let exec_args = args_without_redirect(args);
        let command_output = Command::new(command)
            .args(&exec_args)
            .output()
            .expect("failed to execute the command");

        let stderr = String::from_utf8_lossy(&command_output.stderr);
        if !stderr.is_empty() {
            process_output(&stderr.to_string(), args, true)?;
            return Ok(0);
        }

        let output = format!("{}", String::from_utf8_lossy(&command_output.stdout));
        process_output(&output, args, false)?;
    } else {
        let output = format!("{command}: not found");
        process_output(&output, args, true)?;
    }

    Ok(0)
}
