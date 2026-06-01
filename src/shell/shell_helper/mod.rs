use std::cell::RefCell;

use rustyline::Helper;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;

mod completion;

#[derive(Clone)]
pub struct ShellHepler {
    pub(super) exe_commands_path: Vec<String>,
    last_prefix: RefCell<Option<String>>,
    tab_count: RefCell<usize>,
}

impl ShellHepler {
    pub fn new(exe_commands_path: Vec<String>) -> Self {
        Self {
            exe_commands_path,
            last_prefix: RefCell::new(None),
            tab_count: RefCell::new(0),
        }
    }

    pub fn add_exe_command(&mut self, path: String) {
        self.exe_commands_path.push(path);
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
