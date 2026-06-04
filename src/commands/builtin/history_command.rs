use std::vec::IntoIter;

use super::*;

pub fn handle_history_command(shell: &mut Shell, args: &[String]) -> Result<usize> {
    let mut segmented_args_iter = segment_args(args).into_iter();

    let first_arg = segmented_args_iter.next();
    let mut n = None;

    if let Some(arg) = first_arg {
        if arg == "-r" {
            let path = find_path(&mut segmented_args_iter)?;

            shell.rl.load_history(&path)?;
            return Ok(0);
        } else if arg == "-w" {
            let path = find_path(&mut segmented_args_iter)?;

            shell.write_history_to_file(Some(&path), false)?;
            return Ok(0);
        } else if arg == "-a" {
            let path = find_path(&mut segmented_args_iter)?;

            shell.write_history_to_file(Some(&path), true)?;
            return Ok(0);
        } else {
            n = Some(arg.parse()?);
        }
    }
    let history = shell.history();
    let history_len = history.len();

    let skip = history_len - n.unwrap_or(history_len);
    let history_iter = history.iter().skip(skip);

    let mut output = String::new();

    for (ind, item) in history_iter.enumerate() {
        let number = skip + ind + 1;

        let formatted = format!("\t{number}  {item}\n");
        output.push_str(&formatted);
    }

    process_output(&output, args, false)
}

fn find_path(segmented_args_iter: &mut IntoIter<String>) -> Result<String> {
    segmented_args_iter
        .next()
        .ok_or_else(|| anyhow!("path is not specified"))
}
