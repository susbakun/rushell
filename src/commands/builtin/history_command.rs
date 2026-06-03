use std::fs::{File, OpenOptions};

use super::*;

pub fn handle_history_command(shell: &mut Shell, args: &[String]) -> Result<usize> {
    let mut segmented_args_iter = segment_args(args).into_iter();

    let first_arg = segmented_args_iter.next();
    let mut n = None;

    if let Some(arg) = first_arg {
        if arg == "-r" {
            let path = segmented_args_iter
                .next()
                .ok_or_else(|| anyhow!("path is not specified"))?;

            shell.rl.load_history(&path)?;
            return Ok(0);
        } else if arg == "-w" {
            let path = segmented_args_iter
                .next()
                .ok_or_else(|| anyhow!("path is not specified"))?;

            let mut output = shell
                .history()
                .iter()
                .map(|history| history.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            output.push('\n');

            let file = open_file(&path, false)?;

            write_to_file(&output, file)?;
            return Ok(0);
        } else if arg == "-a" {
            let path = segmented_args_iter
                .next()
                .ok_or_else(|| anyhow!("path is not specified"))?;

            let mut output = shell
                .history()
                .iter()
                .map(|history| history.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            output.push('\n');

            let file = open_file(&path, true)?;
            write_to_file(&output, file)?;
            shell.rl.clear_history()?;
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
