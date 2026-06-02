use super::*;

pub fn handle_complete_command(shell: &mut Shell, args: &[String]) -> Result<usize> {
    let flag = args.first();
    let Some(flag) = flag else { return Ok(0) };

    let mut output = String::new();

    if flag == "-p" {
        let command_name = &args[1];
        if shell.complete_command_registered(&command_name) {
            output = shell.get_formatted_completion_command(&command_name);
        } else {
            output = format!("complete: {command_name}: no completion specification");
        }
    } else if flag == "-C" {
        let (path, command_name) = (&args[1], &args[2]);
        shell.add_complete_command((path, command_name));
    } else if flag == "-r" {
        let command_name = &args[1];
        shell.remove_complete_command(command_name);
    } else {
        return Ok(0);
    }

    process_output(&output, args, false)
}
