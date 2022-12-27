use crate::docker;

pub fn pull_new_version_of_docker_image() -> Result<String, String>
{
    match docker::run_command([
        "pull",
        "ghcr.io/epitech/coding-style-checker:latest"
    ].into()) {
        Ok(output) => Ok(output),
        Err(_) => Err(
            "Impossible de télécharger l'image Docker !".to_string()
        )
    }
}

pub fn clean_olds_versions_of_docker_image() -> Result<String, String>
{
    match docker::run_command([
        "image",
        "prune",
        "-f"
    ].into()) {
        Ok(output) => Ok(output),
        Err(_) => Err(
            "Impossible de supprimer les anciennes versions de l'image Docker !".to_string()
        )
    }
}

pub fn generate_reports_file(current_working_directory: &str) -> Result<String, String>
{
    match docker::run_command([
        "run",
        "--rm",
        "-i",
        "-v",
        &format!("{current_working_directory}:/mnt/delivery"),
        "-v",
        &format!("{current_working_directory}:/mnt/reports"),
        "ghcr.io/epitech/coding-style-checker:latest",
        "/mnt/delivery",
        "/mnt/reports"
    ].into()) {
        Ok(output) => Ok(output),
        Err(_) => Err(
            "Impossible de supprimer les anciennes versions de l'image Docker !".to_string()
        )
    }
}
