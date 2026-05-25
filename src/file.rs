use super::*;
use std::fs::{File, OpenOptions};

pub fn write_to_file(output: &String, mut file: File) -> Result<usize> {
    Ok(file.write(output.as_bytes())?)
}

pub fn find_file(args: &[String], is_stderror: bool) -> Result<File> {
    let redirect_ops = if is_stderror {
        &STDERROR_REDIRECT_OPS
    } else {
        &STDOUT_REDIRECT_OPS
    };

    for i in 0..args.len() {
        if redirect_ops.contains(&args[i].as_str()) {
            let file_name = args
                .get(i + 1)
                .ok_or_else(|| anyhow!("no file name provided"))?;

            return Ok(OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(file_name)?);
        }
    }

    Err(anyhow!("redirect operator not found"))
}
