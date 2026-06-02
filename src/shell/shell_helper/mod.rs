use std::cell::RefCell;
use std::collections::HashMap;

use rustyline::Helper;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;

mod completion;

#[derive(Clone)]
pub struct ShellHepler {
    pub(super) exe_commands: Vec<String>,
    pub(super) exe_paths: Vec<String>,
    pub(super) complete_commands: HashMap<String, String>,
    last_prefix: RefCell<Option<String>>,
    tab_count: RefCell<usize>,
    pub(super) jobs: Vec<String>,
}

impl ShellHepler {
    pub fn new(exe_commands: Vec<String>, exe_paths: Vec<String>) -> Self {
        Self {
            exe_commands,
            exe_paths,
            complete_commands: HashMap::new(),
            last_prefix: RefCell::new(None),
            tab_count: RefCell::new(0),
            jobs: vec![],
        }
    }

    pub fn register_complete_command(&mut self, name: String, completer_path: String) {
        self.complete_commands.insert(name, completer_path);
    }

    pub fn remove_complete_command(&mut self, command_name: &String) {
        self.complete_commands.remove(command_name);
    }

    pub fn complete_command_registered(&self, command_name: &String) -> bool {
        self.complete_commands.contains_key(command_name)
    }

    pub fn get_formatted_completion_command(&self, command_name: &String) -> String {
        let path = self.complete_commands.get(command_name).unwrap();
        format!("complete -C '{path}' {command_name}")
    }

    pub(super) fn tab_press_count(&self, prefix: &str) -> usize {
        let same_prefix = self.last_prefix.borrow().as_deref() == Some(prefix);

        if same_prefix {
            *self.tab_count.borrow_mut() += 1;
        } else {
            *self.tab_count.borrow_mut() = 1;
            *self.last_prefix.borrow_mut() = Some(prefix.to_string());
        }

        *self.tab_count.borrow()
    }

    pub(super) fn add_job(&mut self, job: String) {
        self.jobs.push(job);
    }

    pub(super) fn next_job_id(&mut self) -> usize {
        self.jobs.len()
    }
}

impl Hinter for ShellHepler {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Highlighter for ShellHepler {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        line.into()
    }
}

impl Validator for ShellHepler {
    fn validate(
        &self,
        _ctx: &mut rustyline::validate::ValidationContext,
    ) -> rustyline::Result<rustyline::validate::ValidationResult> {
        Ok(rustyline::validate::ValidationResult::Valid(None))
    }
}

impl Helper for ShellHepler {}
