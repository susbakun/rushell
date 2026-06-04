use super::*;

pub fn handle_declare_command(args: &[String]) -> Result<usize> {
    let segmented_args = segment_args(args);
    let mut output = String::new();

    let flag = &segmented_args[0];

    if flag == "-p" {
        let variable = &segmented_args[1];
        let formatted = format!("declare: {variable}: not found");

        output.push_str(&formatted);
    }

    process_output(&output, args, false)
}
