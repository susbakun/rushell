use std::env;

use super::*;

pub fn handle_cd_command(shell: &Shell, args: &[String]) -> Result<usize> {
    let segmented_args = segment_args(shell, args);

    let path = &segmented_args[0];
    let path = PathBuf::from(path);

    if !path.is_dir() {
        let output = format!("cd: {}: No such file or directory", path.display());
        return process_output(&output, args, true);
    } else {
        env::set_current_dir(path)?;
        return Ok(0);
    }
}
