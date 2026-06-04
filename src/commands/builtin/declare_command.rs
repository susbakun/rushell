use super::*;

pub fn handle_declare_command(shell: &mut Shell, args: &[String]) -> Result<usize> {
    let segmented_args = segment_args(args);
    let mut output = String::new();

    if args[0] == "-p" {
        let variable = &segmented_args[1];
        let formatted;

        if let Some(value) = shell.get_variable(variable) {
            formatted = format!("declare -- {variable}=\"{value}\"");
        } else {
            formatted = format!("declare: {variable}: not found");
        }

        output.push_str(&formatted);
    } else if args[1] == "=" {
        let var = &args[0];
        let value = &args[2];

        shell.add_variable(var, value);

        return Ok(0);
    }

    process_output(&output, args, false)
}
