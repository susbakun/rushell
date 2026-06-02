use super::*;

use std::io::Write;
use std::process::{Command, Stdio};

pub struct PipelineParts {
    pub left_args: Vec<String>,
    pub right_command: String,
    pub right_args: Vec<String>,
}

pub fn run_pipeline(shell: &mut Shell, left_command: &str, args: &[String]) -> Result<()> {
    let parts = split_pipeline(args)?;

    if is_builtin_command(left_command) {
        run_builtin_left(shell, left_command, &parts)?;
    } else if is_builtin_command(&parts.right_command) {
        run_builtin_right(shell, &parts)?;
    } else {
        run_external_pipeline(shell, left_command, &parts)?;
    }

    Ok(())
}

fn run_builtin_left(shell: &Shell, left_command: &str, parts: &PipelineParts) -> Result<()> {
    let left_args = segment_args(&parts.left_args);
    let remainder = left_args.join(" ");

    let output = match left_command {
        "echo" => format_echo_output(&remainder),
        "type" => format_type_output(shell.paths(), &remainder)?,
        _ => return Ok(()),
    };

    spawn_with_stdin(
        &parts.right_command,
        &parts.right_args,
        output.as_bytes(),
        shell.paths(),
    )?;
    Ok(())
}

fn run_builtin_right(shell: &Shell, parts: &PipelineParts) -> Result<usize> {
    let right_args: Vec<String> = parts.right_args.clone();
    let remainder = right_args.join(" ");
    let output = match parts.right_command.as_str() {
        "echo" => format_echo_output(&remainder),
        "type" => format_type_output(shell.paths(), &remainder)?,
        "exit" => {
            handle_exit_command();
            return Ok(0);
        }
        _ => return Ok(0),
    };

    process_output(&output, &right_args, false)
}

fn run_external_pipeline(shell: &Shell, left_command: &str, parts: &PipelineParts) -> Result<()> {
    let paths = shell.paths();
    let left_args = segment_args(&parts.left_args);

    if !find_exe(paths, left_command)?.1 {
        let output = format!("{left_command}: not found\n");
        print!("{output}");
        io::stdout().flush()?;
        return Ok(());
    }

    if !find_exe(paths, &parts.right_command)?.1 {
        let output = format!("{}: not found\n", parts.right_command);
        print!("{output}");
        io::stdout().flush()?;
        return Ok(());
    }

    let mut left = Command::new(left_command)
        .args(&left_args)
        .stdout(Stdio::piped())
        .spawn()?;

    let mut right = Command::new(&parts.right_command)
        .args(&parts.right_args)
        .stdin(left.stdout.take().unwrap())
        .spawn()?;

    right.wait()?;
    left.wait()?;
    Ok(())
}

fn spawn_with_stdin(command: &str, args: &[String], input: &[u8], paths: &[String]) -> Result<()> {
    if !find_exe(paths, command)?.1 {
        let output = format!("{command}: not found\n");
        print!("{output}");
        io::stdout().flush()?;
        return Ok(());
    }

    let mut child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .spawn()?;

    child.stdin.as_mut().unwrap().write_all(input)?;
    drop(child.stdin.take());
    child.wait()?;
    Ok(())
}
