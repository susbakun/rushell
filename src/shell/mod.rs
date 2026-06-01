use super::*;

use std::collections::HashMap;

mod shell_helper;
use shell_helper::*;

pub struct Shell {
    // name -> path
    complete_commands: HashMap<String, String>,
    helper: ShellHepler,
}

impl Shell {
    pub fn new() -> Result<Self> {
        let paths = std::env::var("PATH").unwrap_or_default();
        let paths = paths
            .split(":")
            .map(|path| path.to_string())
            .collect::<Vec<String>>();

        let exe_commands = find_command_names_on_path(&paths)?;

        let helper = ShellHepler::new(exe_commands);

        Ok(Self {
            complete_commands: HashMap::new(),
            helper,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let config = Config::builder()
            .completion_type(CompletionType::List)
            .build();
        let mut rl = Editor::<ShellHepler, DefaultHistory>::with_config(config)?;
        rl.set_helper(Some(self.helper.clone()));

        loop {
            let readline = rl.readline("$ ");

            match readline {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    }

                    let tokens = parse_input(&line);
                    if tokens.is_empty() {
                        continue;
                    }
                    let command = &tokens[0];
                    let args = tokens.get(1..).unwrap_or_default();

                    let remainder = args_without_redirect(args).join(" ");

                    process_command(
                        self,
                        command,
                        &remainder,
                        args,
                        &&self.helper.exe_commands_path.clone(),
                    )?;
                }
                Err(Interrupted) => break Ok(()),
                Err(err) => break Err(anyhow!("{err}")),
            }
        }
    }

    pub fn add_complete_command(&mut self, command: (&String, &String)) {
        let (path, name) = command;

        self.complete_commands.insert(name.clone(), path.clone());

        self.helper.add_exe_command(path.clone());
    }

    pub fn complete_command_registered(&self, command_name: &String) -> bool {
        self.complete_commands.contains_key(command_name)
    }

    pub fn get_formatted_completion_command(&self, command_name: &String) -> String {
        // we already checked the existence of command
        let path = self.complete_commands.get(command_name).unwrap();

        format!("complete -C '{path}' {command_name}")
    }
}
