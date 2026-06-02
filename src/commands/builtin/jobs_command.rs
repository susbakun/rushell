use super::*;

pub fn handle_jobs_command(shell: &mut Shell, args: &[String], only_done: bool) -> Result<usize> {
    shell.refresh_jobs();

    let mut output = String::new();
    let jobs = shell.jobs();

    print_jobs(jobs, only_done, &mut output);

    shell.reap_finished_jobs();

    process_output(&output, args, false)
}

fn print_jobs(jobs: &Vec<Job>, only_done: bool, output: &mut String) {
    let n = jobs.len();

    for (ind, job) in jobs.iter().enumerate() {
        if only_done && !job.is_job_finished() {
            continue;
        }

        let padd = " ".repeat(17);
        let sign = get_sign(ind, n);

        let job_command = if job.is_job_finished() {
            job.get_job_command().to_string()
        } else {
            format!("{} &", job.get_job_command())
        };
        let job_status_str = job.get_job_status();
        let job_number = job.get_job_number();

        let formatted = format!("[{job_number}]{sign}  {job_status_str}{padd}{job_command}\n");
        output.push_str(&formatted);
    }
}

fn get_sign(ind: usize, n: usize) -> String {
    let sign = if ind == n - 1 {
        "+"
    } else if ind == n - 2 {
        "-"
    } else {
        " "
    };

    sign.into()
}
