use std::process::Stdio;

use super::*;

pub fn process_output(
    output: &String,
    args: &[String],
    is_stderror: bool,
    paths: &Vec<String>,
) -> Result<usize> {
    if is_piped(args) {
        return output_piped_command(output, args, paths);
    }

    if has_std_redirect(args)
        || has_err_redirect(args)
        || has_std_append(args)
        || has_err_append(args)
    {
        let file = find_file(args, false)?;
        if should_redirect(args, is_stderror) {
            write_to_file(output, file)?;
            return Ok(0);
        }
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

pub fn output_piped_command(
    output: &String,
    args: &[String],
    paths: &Vec<String>,
) -> Result<usize> {
    let (second_command, second_command_args) = get_command_after_pipe(args)?;
    let second_command_args = second_command_args
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>();

    let remainder = second_command_args.join(" ");

    if second_command == "exit" {
        handle_exit_command();
    } else if second_command == "echo" {
        return handle_echo_command(&remainder, paths, &second_command_args);
    } else if second_command == "type" {
        return handle_type_command(&remainder, paths, &second_command_args);
    }

    let mut second = Command::new(second_command)
        .args(second_command_args)
        .stdin(Stdio::piped())
        .spawn()?;

    second
        .stdin
        .as_mut()
        .unwrap()
        .write_all(output.as_bytes())?;

    drop(second.stdin.take());

    second.wait()?;
    return Ok(0);
}

pub fn should_redirect(args: &[String], is_stderror: bool) -> bool {
    if is_stderror {
        return has_err_redirect(args) || has_err_append(args);
    } else {
        return has_std_redirect(args) || has_std_append(args);
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

pub fn has_std_append(args: &[String]) -> bool {
    args.iter()
        .any(|arg| STDOUT_APPEND_OPS.contains(&arg.as_str()))
}

pub fn has_err_append(args: &[String]) -> bool {
    args.iter()
        .any(|arg| STDERROR_APPEND_OPS.contains(&arg.as_str()))
}

pub fn should_truncate_file(args: &[String]) -> bool {
    has_std_redirect(args) || has_err_redirect(args)
}
