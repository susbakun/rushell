use super::*;

pub fn process_command(command: &str, remainder: &String, paths: &Vec<&str>) -> Result<()> {
    if command == "exit" {
        handle_exit_command();
    } else if command == "echo" {
        handle_echo_command(remainder);
    } else if command == "type" {
        handle_type_command(remainder, &paths)?;
    } else {
        handle_executable_command(remainder, command, &paths)?;
    }

    Ok(())
}

fn handle_exit_command() {
    std::process::exit(0);
}

fn handle_echo_command(remainder: &String) {
    println!("{remainder}")
}

fn handle_type_command(remainder: &String, paths: &Vec<&str>) -> Result<()> {
    if KNOWN_COMMANDS.contains(&&remainder[..]) {
        println!("{remainder} is a shell builtin");
    } else {
        let (file_path, found) = find_exe(paths, &remainder)?;
        if found {
            let file_path = file_path.unwrap();

            println!("{remainder} is {}", file_path.to_str().unwrap());
        } else {
            println!("{remainder}: not found");
        }
    }

    Ok(())
}

fn handle_executable_command(remainder: &String, command: &str, paths: &Vec<&str>) -> Result<()> {
    let (_, found) = find_exe(paths, command)?;
    let args = remainder.split(" ").collect::<Vec<&str>>();
    if found {
        let output = Command::new(command)
            .args(args)
            .output()
            .expect("failed to execute the program");

        print!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("{command}: not found");
    }

    Ok(())
}
