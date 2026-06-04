use super::*;

mod shell_helper;
pub use shell_helper::*;

pub struct Shell {
    helper: ShellHepler,
    pub rl: RLType,
}

impl Shell {
    pub fn new() -> Result<Self> {
        let paths = std::env::var("PATH").unwrap_or_default();
        let paths = paths
            .split(":")
            .map(|path| path.to_string())
            .collect::<Vec<String>>();

        let hist_path = std::env::var("HISTFILE").ok();

        let exe_commands = find_command_names_on_path(&paths)?;
        let helper = ShellHepler::new(exe_commands, paths, &hist_path);

        let mut rl = Self::setup_rl(&helper)?;

        if let Some(hist_path) = &hist_path {
            rl.load_history(&hist_path)?;
        }

        Ok(Self { helper, rl })
    }

    fn setup_rl(helper: &ShellHepler) -> Result<RLType> {
        let config = Config::builder()
            .completion_type(CompletionType::List)
            .history_ignore_space(false)
            .build();
        let mut rl = RLType::with_config(config)?;
        rl.set_helper(Some(helper.clone()));

        Ok(rl)
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            // reap before the next prompt
            handle_jobs_command(self, &[], true)?;

            let readline = self.rl.readline("$ ");

            match readline {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    }

                    let tokens = parse_input(&line);
                    if tokens.is_empty() {
                        continue;
                    }

                    self.add_command_to_history(&line)?;

                    let command = &tokens[0];
                    let args = tokens.get(1..).unwrap_or_default();

                    let remainder = segment_args(args).join(" ");

                    process_command(self, command, &remainder, args, &tokens)?;

                    if let Some(helper) = self.rl.helper_mut() {
                        *helper = self.helper.clone();
                    }
                }
                Err(Interrupted) => break Ok(()),
                Err(err) => break Err(anyhow!("{err}")),
            }
        }
    }

    pub fn paths(&self) -> &[String] {
        &self.helper.exe_paths
    }

    pub fn add_complete_command(&mut self, command: (&String, &String)) {
        let (completer_path, name) = command;
        self.helper
            .register_complete_command(name.clone(), completer_path.clone());
    }

    pub fn remove_complete_command(&mut self, command_name: &String) {
        self.helper.remove_complete_command(command_name);
    }

    pub fn complete_command_registered(&self, command_name: &String) -> bool {
        self.helper.complete_command_registered(command_name)
    }

    pub fn get_formatted_completion_command(&self, command_name: &String) -> String {
        self.helper.get_formatted_completion_command(command_name)
    }

    pub fn add_job(&mut self, job: Job) {
        self.helper.add_job(job);
    }

    pub fn refresh_jobs(&mut self) {
        self.helper.refresh_jobs();
    }

    pub fn reap_finished_jobs(&mut self) {
        self.helper.reap_finished_jobs();
    }

    pub fn jobs(&self) -> &Vec<Job> {
        self.helper.jobs()
    }

    pub fn next_job_number(&mut self) -> usize {
        self.helper.next_job_number()
    }

    pub fn history(&self) -> Vec<&String> {
        self.rl.history().iter().collect()
    }

    fn add_command_to_history(&mut self, line: &String) -> Result<()> {
        self.rl.add_history_entry(line)?;
        Ok(())
    }

    pub fn write_history_to_file(&mut self, path: Option<&String>, append: bool) -> Result<()> {
        let path = if let Some(path) = path {
            path
        } else if let Some(path) = &self.helper.hist_path {
            path
        } else {
            return Err(anyhow!("no path is provided"));
        };

        let mut output = self
            .history()
            .iter()
            .map(|history| history.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        output.push('\n');

        let file = open_file(&path, append)?;

        write_to_file(&output, file)?;

        // clearing history on append mode
        if append {
            self.rl.clear_history()?;
        }

        Ok(())
    }

    pub fn get_variable(&self, key: &String) -> Option<&String> {
        self.helper.get_variable(key)
    }

    pub fn add_variable(&mut self, key: &String, value: &String) {
        self.helper.add_variable(key, value);
    }
}
