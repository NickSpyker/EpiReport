mod docker;
mod epitech;
mod tools;

fn main() -> Result<(), String>
{
    let current_working_directory: String = tools::get_current_working_directory()?;

    tools::remove_file_if_exists(&current_working_directory, "coding-style-reports.log");

    epitech::pull_new_version_of_docker_image()?;

    epitech::clean_olds_versions_of_docker_image()?;

    epitech::generate_reports_file(&current_working_directory)?;

    Ok(())
}
