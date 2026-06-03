use super::*;

pub fn handle_executable_command(
    shell: &mut Shell,
    args: &[String],
    command: &str,
) -> Result<usize> {
    let paths = shell.paths();
    let (_, found) = find_exe(paths, command)?;

    let is_background_job = args.contains(&"&".to_string());
    let exec_args = segment_args(args);

    if !found {
        let output = format!("{command}: not found");
        return process_output(&output, args, true);
    }

    if is_background_job {
        return handle_background_job(shell, command, exec_args, args);
    }

    let command_output = Command::new(command)
        .args(&exec_args)
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

fn handle_background_job(
    shell: &mut Shell,
    command: &str,
    exec_args: Vec<String>,
    args: &[String],
) -> Result<usize> {
    let job_command = if exec_args.is_empty() {
        command.to_string()
    } else {
        format!("{} {}", command, exec_args.join(" "))
    };

    let child = Command::new(command)
        .args(&exec_args)
        .spawn()
        .expect("failed to execute the command");
    let job_id = child.id();

    let number = shell.next_job_number();
    let job = Job::new(number, job_command, child);

    shell.add_job(job);

    let output = format!("[{}] {}", number, job_id);
    return process_output(&output, args, false);
}
