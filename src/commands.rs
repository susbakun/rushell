use super::*;

pub fn process_command(command: &str, remainder: &str, paths: &Vec<&str>) -> Result<()> {
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

fn process_remainder(remainder: &str) -> String {
    let mut formated = String::new();
    let mut single_quote = false;
    let mut space_count = 0;

    for ch in remainder.chars() {
        if ch == '\'' {
            single_quote = !single_quote;
            space_count = 0;
        } else if single_quote {
            formated.push(ch);
            space_count = 0;
        } else if ch == ' ' {
            space_count += 1;
            if space_count < 2 {
                formated.push(ch);
            }
        } else {
            formated.push(ch);
            space_count = 0;
        }
    }

    formated
}

fn handle_exit_command() {
    std::process::exit(0);
}

fn handle_echo_command(remainder: &str) {
    let remainder = process_remainder(remainder);

    println!("{remainder}")
}

fn handle_type_command(remainder: &str, paths: &Vec<&str>) -> Result<()> {
    let remainder = process_remainder(remainder);

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

fn handle_executable_command(remainder: &str, command: &str, paths: &Vec<&str>) -> Result<()> {
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
