use super::*;

pub fn handle_history_command(shell: &Shell, args: &[String]) -> Result<usize> {
    let history = shell.history();
    let mut output = String::new();

    for (ind, item) in history.iter().enumerate() {
        let number = ind + 1;

        let formatted = format!("\t{number}  {item}\n");
        output.push_str(&formatted);
    }

    process_output(&output, args, false)
}
