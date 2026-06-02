use super::*;
use std::{cell::RefCell, process::Child, rc::Rc};

#[derive(Clone, PartialEq)]
pub enum JobStatus {
    Running,
    Done,
}

impl From<&JobStatus> for String {
    fn from(value: &JobStatus) -> Self {
        match value {
            JobStatus::Running => String::from("Running"),
            JobStatus::Done => String::from("Done"),
        }
    }
}

#[derive(Clone)]
pub struct Job {
    status: JobStatus,
    command: String,
    child: Rc<RefCell<Child>>,
}

impl Job {
    pub fn new(command: String, child: Child) -> Self {
        Job {
            status: JobStatus::Running,
            command,
            child: Rc::new(RefCell::new(child)),
        }
    }

    pub fn get_job_command(&self) -> &String {
        &self.command
    }

    pub fn update_status(&mut self) -> Result<()> {
        match self.child.borrow_mut().try_wait()? {
            Some(_) => self.status = JobStatus::Done,
            None => {}
        }
        Ok(())
    }

    pub fn get_job_status(&self) -> String {
        (&self.status).into()
    }

    pub fn is_job_finished(&self) -> bool {
        self.status == JobStatus::Done
    }
}
