use super::*;

pub fn handle_echo_command(remainder: &str, args: &[String]) -> Result<usize> {
    process_output(&format_echo_output(remainder), args, false)
}
