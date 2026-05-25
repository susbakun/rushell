use super::*;
use std::fs::{File, OpenOptions};

pub fn write_to_file(output: &String, mut file: File) -> Result<usize> {
    Ok(file.write(output.as_bytes())?)
}

pub fn find_file(args: &[String]) -> Result<File> {
    let joined_args = args.join(" ");

    let file_name = joined_args
        .split(">")
        .map(|item| item.trim())
        .nth(1)
        .ok_or_else(|| anyhow!("no file name provided"))?;

    // Create parent directories if they don't exist
    if let Some(parent) = Path::new(file_name).parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name)?)
}
