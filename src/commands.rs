use anyhow::Ok;

use super::*;

pub fn process_command(
    shell: &mut Shell,
    command: &str,
    remainder: &str,
    args: &[String],
    paths: &Vec<String>,
) -> Result<()> {
    if command == "exit" {
        handle_exit_command();
    } else if command == "echo" {
        handle_echo_command(remainder, args)?;
    } else if command == "type" {
        handle_type_command(remainder, paths, args)?;
    } else if command == "complete" {
        handle_complete_command(shell, args)?;
    } else if command == "jobs" {
        handle_job_command(shell, args)?;
    } else {
        handle_executable_command(shell, args, command, &paths)?;
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

fn handle_type_command(remainder: &str, paths: &Vec<String>, args: &[String]) -> Result<usize> {
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

fn handle_complete_command(shell: &mut Shell, args: &[String]) -> Result<usize> {
    let flag = args.first();
    let Some(flag) = flag else { return Ok(0) };

    let mut output = String::new();

    if flag == "-p" {
        let command_name = &args[1];
        if shell.complete_command_registered(&command_name) {
            output = shell.get_formatted_completion_command(&command_name);
        } else {
            output = format!("complete: {command_name}: no completion specification");
        }
    } else if flag == "-C" {
        let (path, command_name) = (&args[1], &args[2]);
        shell.add_complete_command((path, command_name));
    } else if flag == "-r" {
        let command_name = &args[1];
        shell.remove_complete_command(command_name);
    } else {
        return Ok(0);
    }

    process_output(&output, args, true)
}

fn handle_job_command(shell: &mut Shell, args: &[String]) -> Result<usize> {
    let mut output = String::new();
    let n = shell.get_jobs().len();

    for (ind, job) in shell.get_jobs().iter().enumerate() {
        let job_number = ind + 1;
        let padd = " ".repeat(17);
        let sign = if ind == n - 1 {
            // most recent
            "+"
        } else if ind == n - 2 {
            // second most recent
            "-"
        } else {
            // otherwise
            " "
        };

        let formatted = format!("[{job_number}]{}  Running{}{}\n", sign, padd, job);
        output.push_str(&formatted);
    }

    process_output(&output, args, false)
}

fn handle_executable_command(
    shell: &mut Shell,
    args: &[String],
    command: &str,
    paths: &Vec<String>,
) -> Result<usize> {
    let (_, found) = find_exe(paths, command)?;

    let is_background_job = args.contains(&"&".to_string());

    let exec_args = parse_args(args);

    if !found {
        let output = format!("{command}: not found");
        return process_output(&output, args, true);
    }

    if is_background_job {
        let child = Command::new(command)
            .args(exec_args)
            .spawn()
            .expect("failed to execute the command");

        let joined_args = args.join(" ");
        let job_command = format!("{command} {joined_args}");

        shell.add_job(job_command);

        let output = format!("[{}] {}", shell.get_job_number(), child.id());
        return process_output(&output, args, false);
    }

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

    Ok(0)
}
