use super::*;

pub fn handle_history_command(shell: &Shell, args: &[String]) -> Result<usize> {
    let history = shell.history();
    let segmented_args = segment_args(args);

    let history_len = history.len();

    let n_str = segmented_args.get(0).map(String::from).unwrap_or_default();
    let mut n: usize = history_len;

    if !n_str.is_empty() {
        n = n_str.parse()?;
    }

    let skip = history_len - n;
    let history_iter = history.iter().skip(skip);

    let mut output = String::new();

    for (ind, item) in history_iter.enumerate() {
        let number = skip + ind + 1;

        let formatted = format!("\t{number}  {item}\n");
        output.push_str(&formatted);
    }

    process_output(&output, args, false)
}
