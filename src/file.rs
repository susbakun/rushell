use super::*;
use std::fs::{File, OpenOptions};

pub fn write_to_file(output: &String, mut file: File) -> Result<usize> {
    Ok(file.write(output.as_bytes())?)
}

pub fn find_file(args: &[String]) -> Result<File> {
    for i in 0..args.len() {
        if STDOUT_REDIRECT_OPS.contains(&args[i].as_str())
            || STDERROR_REDIRECT_OPS.contains(&args[i].as_str())
        {
            let file_name = args
                .get(i + 1)
                .ok_or_else(|| anyhow!("no file name provided"))?;

            return Ok(OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(file_name)?);
        }
    }

    Err(anyhow!("no redirect operator found"))
}
