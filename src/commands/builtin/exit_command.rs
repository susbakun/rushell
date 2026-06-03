use super::*;

pub fn handle_exit_command(shell: &mut Shell) -> Result<()> {
    shell.write_history_to_file(None, false)?;
    std::process::exit(0);
}
