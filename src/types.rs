use rustyline::{Editor, history::DefaultHistory};

use crate::shell::ShellHepler;

pub type RLType = Editor<ShellHepler, DefaultHistory>;
