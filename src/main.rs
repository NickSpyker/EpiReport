mod docker;
mod parser;
mod tree;

use std::path::Path;
use crate::parser::Log;

fn main() -> Result<(), String>
{
    let buffer: String = match Path::new(".").canonicalize() {
        Ok(dir) => {
            println!("Chemin du repertoire courant : {}", dir.display());
            docker::get_coding_style_reports(&dir)?
        },
        Err(_) => return Err("Impossible de recuperer le chemin du repertoire courant !".to_string())
    };

    tree::display(Log::parse(buffer)?)?;

    Ok(())
}
