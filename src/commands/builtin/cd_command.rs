use std::env;

use super::*;

pub fn handle_cd_command(shell: &Shell, args: &[String]) -> Result<usize> {
    let segmented_args = segment_args(shell, args);

    let path_str = &segmented_args[0];
    let mut path = PathBuf::from(path_str);

    if !path.is_dir() && !is_root_dir(path_str) {
        let output = format!("cd: {}: No such file or directory", path.display());
        return process_output(&output, args, true);
    } else {
        if is_root_dir(path_str) {
            path = std::env::home_dir().ok_or_else(|| anyhow!("home path is not specified"))?;
        }
        env::set_current_dir(path)?;
        return Ok(0);
    }
}

fn is_root_dir(path_str: &String) -> bool {
    path_str == "~"
}
