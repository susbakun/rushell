use std::process::Stdio;

use super::*;

pub fn process_output(output: &String, args: &[String], is_stderror: bool) -> Result<usize> {
    if is_piped(args) {
        let (second_command, second_command_args) = get_command_after_pipe(args)?;

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
