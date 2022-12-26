mod docker;

use std::{
    path::{PathBuf, Path},
};

fn main() -> Result<(), String>
{
    let current_directory: PathBuf = match Path::new(".").canonicalize() {
        Ok(dir) => {
            println!("Chemin du repertoire courant : {}", dir.display());
            dir
        },
        Err(_) => return Err("ERREUR : Impossible de recuperer le chemin du repertoire courant !".to_string())
    };

    let buffer: String = docker::get_coding_style_reports(current_directory)?;

    println!("{buffer}");

    Ok(())
}
