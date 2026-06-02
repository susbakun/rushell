use anyhow::Ok;

use super::*;

mod pipeline;
pub use pipeline::*;
mod builtin;
pub use builtin::*;
mod external_command;
pub use external_command::*;

pub fn process_command(
    shell: &mut Shell,
    command: &str,
    remainder: &str,
    args: &[String],
    tokens: &Vec<String>,
) -> Result<()> {
    if is_piped(args) {
        return run_pipeline(shell, tokens);
    }

    if command == "exit" {
        handle_exit_command();
    } else if command == "echo" {
        handle_echo_command(remainder, args)?;
    } else if command == "type" {
        handle_type_command(remainder, shell, args)?;
    } else if command == "complete" {
        handle_complete_command(shell, args)?;
    } else if command == "jobs" {
        handle_jobs_command(shell, args, false)?;
    } else {
        handle_executable_command(shell, args, command)?;
    }

    Ok(())
}
