use super::*;

use std::io::Write;
use std::process::{Child, ChildStdout, Command, Stdio};

pub struct PipelineParts {
    pub command: String,
    pub args: Vec<String>,
}

enum PipelineInput {
    Text(String),
    Stdout(ChildStdout),
}

pub fn run_pipeline(shell: &mut Shell, tokens: &Vec<String>) -> Result<()> {
    let parts = split_pipeline(tokens)?;

    let mut prev_stdout: Option<PipelineInput> = None;

    let mut children = vec![];
    let n = parts.len();

    for (ind, part) in parts.iter().enumerate() {
        let last = ind == n - 1;

        if is_builtin_command(&part.command) {
            let stdout = run_builtin(shell, &part)?;
            prev_stdout = Some(PipelineInput::Text(stdout));
        } else {
            let (child, stdout) = run_external_pipeline(shell, &part, prev_stdout, last)?;
            prev_stdout = stdout.map(PipelineInput::Stdout);
            children.push(child);
        }
    }

    for child in children.iter_mut() {
        child.wait()?;
    }

    Ok(())
}

fn run_builtin(shell: &Shell, part: &PipelineParts) -> Result<String> {
    let args = &part.args;
    let remainder = args.join(" ");

    let output = match part.command.as_str() {
        "echo" => format_echo_output(&remainder),
        "type" => format_type_output(shell.paths(), &remainder)?,
        _ => String::new(),
    };

    Ok(output)
}

fn run_external_pipeline(
    shell: &Shell,
    part: &PipelineParts,
    prev_stdout: Option<PipelineInput>,
    last: bool,
) -> Result<(Child, Option<ChildStdout>)> {
    let paths = shell.paths();
    let args = &part.args;
    let command = &part.command;

    if !find_exe(paths, &part.command)?.1 {
        let output = format!("{}: not found\n", part.command);
        print!("{output}");
        io::stdout().flush()?;
        return Err(anyhow!("didn't find the command"));
    }

    let mut command = Command::new(&command);
    let mut prev_text = String::new();

    match prev_stdout {
        Some(PipelineInput::Stdout(stdout)) => {
            command.stdin(Stdio::from(stdout));
        }
        Some(PipelineInput::Text(text)) => {
            prev_text = text;
            command.stdin(Stdio::piped());
        }
        None => {}
    }

    if !last {
        command.stdout(Stdio::piped());
    }

    let mut child = command.args(args).spawn()?;

    if !prev_text.is_empty() {
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(prev_text.as_bytes())?;
    }

    let new_stdout = child.stdout.take();

    Ok((child, new_stdout))
}
