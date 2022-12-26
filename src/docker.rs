use std::{
    path::PathBuf,
    fs::{remove_file, read_to_string},
    process::Command
};

pub fn get_coding_style_reports(path: &PathBuf) -> Result<String, String>
{
    let reports_file_path: String = format!("{}/coding-style-reports.log", path.display());

    remove_file(&reports_file_path).unwrap_or(());

    match Command::new("sudo").args([
        "docker",
        "pull",
        "ghcr.io/epitech/coding-style-checker:latest"
    ]).output() {
        Ok(output) => println!("{}", String::from_utf8_lossy(&output.stdout)),
        Err(_) => return Err("ERREUR : Impossible de telecharger l image Docker !".to_string())
    }

    match Command::new("sudo").args([
        "docker",
        "image",
        "prune",
        "-f"
    ]).output() {
        Ok(output) => println!("{}", String::from_utf8_lossy(&output.stdout)),
        Err(_) => return Err("ERREUR : Impossible de supprimer les anciennes versions de l image Docker".to_string())
    }

    match Command::new("sudo").args([
        "docker",
        "run",
        "--rm",
        "-i",
        "-v",
        &format!("{}:/mnt/delivery", path.display()),
        "-v",
        &format!("{}:/mnt/reports", path.display()),
        "ghcr.io/epitech/coding-style-checker:latest",
        "/mnt/delivery",
        "/mnt/reports"
    ]).output() {
        Ok(_) => match read_to_string(reports_file_path) {
            Ok(buffer) => Ok(buffer),
            Err(_) => Err("ERREUR : Impossible de lire les rapports !".to_string())
        },
        Err(_) => Err("ERREUR : Impossible de generer les rapports !".to_string())
    }
}
