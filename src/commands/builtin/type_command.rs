use super::*;

pub fn handle_type_command(remainder: &str, shell: &Shell, args: &[String]) -> Result<usize> {
    let output = format_type_output(shell.paths(), remainder)?;
    let is_stderror = output.contains(": not found");
    process_output(&output, args, is_stderror)
}
