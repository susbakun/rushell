use std::env;

use super::*;

pub fn handle_pwd_command(args: &[String]) -> Result<usize> {
    let path = env::current_dir()?;
    let output = path.display().to_string();

    process_output(&output, args, false)
}
