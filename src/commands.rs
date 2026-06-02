use std::process::Stdio;

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
        handle_jobs_command(shell, args, false)?;
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

pub fn handle_jobs_command(shell: &mut Shell, args: &[String], only_done: bool) -> Result<usize> {
    shell.refresh_jobs();

    let mut output = String::new();
    let jobs = shell.jobs();
    let n = jobs.len();

    for (ind, job) in jobs.iter().enumerate() {
        if only_done && !job.is_job_finished() {
            continue;
        }

        let padd = " ".repeat(17);
        let sign = if ind == n - 1 {
            "+"
        } else if ind == n - 2 {
            "-"
        } else {
            " "
        };

        let job_command = if job.is_job_finished() {
            job.get_job_command().to_string()
        } else {
            format!("{} &", job.get_job_command())
        };
        let job_status_str = job.get_job_status();
        let job_number = job.get_job_number();

        let formatted = format!("[{job_number}]{sign}  {job_status_str}{padd}{job_command}\n");
        output.push_str(&formatted);
    }

    shell.reap_finished_jobs();

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
    let is_piped_coomand = args.contains(&"|".to_string());

    let exec_args = parse_args(args);

    if !found {
        let output = format!("{command}: not found");
        return process_output(&output, args, true);
    }

    if is_background_job {
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

    if is_piped_coomand {
        let mut first = Command::new(command)
            .args(&exec_args)
            .stdout(Stdio::piped())
            .spawn()?;

        let first_stdout = first.stdout.take().unwrap();

        let second_part = args.split(|st| st == "|").nth(1).unwrap();
        let second_command = second_part.get(0).unwrap();

        let mut second = Command::new(second_command)
            .stdin(Stdio::from(first_stdout))
            .spawn()?;

        second.wait()?;
        first.wait()?;
        return Ok(0);
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
