use std::{fs, env};

pub fn remove_file_if_exists(current_working_directory: &str, path: &str)
{
    fs::remove_file(format!("{current_working_directory}/{path}")).unwrap_or(());
}

pub fn get_current_working_directory() -> Result<String, String>
{
    match env::current_dir() {
        Ok(path) => Ok(path.to_string_lossy().into_owned()),
        Err(_) => Err("Impossible de récupérer le chemin du répertoire courant !".to_string())
    }
}
