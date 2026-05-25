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
    process_output(&output, args)
}

fn handle_type_command(remainder: &str, paths: &Vec<&str>, args: &[String]) -> Result<usize> {
    if KNOWN_COMMANDS.contains(&&remainder[..]) {
        let output = format!("{remainder} is a shell builtin");
        process_output(&output, args)?;
    } else {
        let (file_path, found) = find_exe(paths, &remainder)?;
        if found {
            let file_path = file_path.unwrap();

            let output = format!("{remainder} is {}", file_path.to_str().unwrap());
            process_output(&output, args)?;
        } else {
            println!("{remainder}: not found");
        }
    }

    Ok(0)
}

fn handle_executable_command(args: &[String], command: &str, paths: &Vec<&str>) -> Result<usize> {
    let (_, found) = find_exe(paths, command)?;
    if found {
        let command_output = Command::new(command)
            .args(args)
            .output()
            .expect("failed to execute the program");

        let output = format!("{}", String::from_utf8_lossy(&command_output.stdout));
        process_output(&output, args)?;
    } else {
        println!("{command}: not found");
    }

    Ok(0)
}
