use super::*;
use std::fs::{File, OpenOptions};

pub fn write_to_file(output: &String, args: &[String]) -> Result<usize> {
    let mut file = find_file(args)?;

    Ok(file.write(output.as_bytes())?)
}

pub fn find_file(args: &[String]) -> Result<File> {
    let joined_args = args.join(" ");

    let file_name = joined_args
        .split(">")
        .map(|item| item.trim())
        .nth(1)
        .ok_or_else(|| anyhow!("no file name provided"))?;

    Ok(OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name)?)
}
