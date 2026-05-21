use super::*;

pub fn find_exe(paths: &Vec<&str>, command: &str) -> Result<(Option<PathBuf>, bool)> {
    let mut found = false;
    for path in paths {
        let dir = fs::read_dir(path)?;
        for file in dir {
            let file = file?;
            let file_name = file.file_name();

            let path = file.path();
            let file_path = Path::new(&path);

            if file_path.is_executable() && file_name == command {
                found = true;
                let file_path = file_path.to_owned();
                return Ok((Some(file_path), found));
            }
        }
    }

    Ok((None, found))
}
