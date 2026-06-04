use super::*;

pub fn handle_declare_command(shell: &mut Shell, args: &[String]) -> Result<usize> {
    let segmented_args = segment_args(args);
    let mut output = String::new();

    if segmented_args[0] == "-p" {
        let variable = &segmented_args[1];
        let formatted;

        if let Some(value) = shell.get_variable(variable) {
            formatted = format!("declare -- {variable}=\"{value}\"");
        } else {
            formatted = format!("declare: {variable}: not found");
        }

        output.push_str(&formatted);
    } else if segmented_args[0].contains("=") {
        let items = segmented_args[0].split("=").collect::<Vec<&str>>();

        let (var, value) = (items[0], items[1]);

        shell.add_variable(var, value);

        return Ok(0);
    }

    process_output(&output, args, false)
}
