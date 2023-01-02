mod tools;
mod epitech;
mod docker;
mod parser;
mod display;

use crate::parser::logs::Log;

const REPORT_FILE_NAME: &str = "coding-style-reports.log";

fn main() -> Result<(), String>
{
    let current_working_directory: String = tools::get_current_working_directory()?;

    tools::remove_file_if_exists(&current_working_directory, REPORT_FILE_NAME);

    epitech::pull_new_version_of_docker_image()?;
    epitech::clean_olds_versions_of_docker_image()?;
    epitech::generate_reports_file(&current_working_directory)?;

    let buffer: String = tools::read_file(&current_working_directory, REPORT_FILE_NAME)?;

    let parsed_logs: Vec<Log> = Log::build_log(buffer);

    for log in parsed_logs {

        display::print(log);
    }

    tools::remove_file_if_exists(&current_working_directory, REPORT_FILE_NAME);

    Ok(())
}
