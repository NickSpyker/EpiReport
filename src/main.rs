mod docker;
mod parser;
mod tree;

use std::path::Path;
use crate::{parser::Log, tree::TreeNodes};

fn main() -> Result<(), String>
{
    let mut current_dir: String = "Unknown".to_string();

    let buffer: String = match Path::new(".").canonicalize() {
        Ok(dir) => {
            current_dir = format!("{}", dir.display());
            docker::get_coding_style_reports(&dir)?
        },
        Err(_) => return Err("Impossible de recuperer le chemin du repertoire courant !".to_string())
    };

    let tree_nodes: TreeNodes = tree::build(Log::parse(buffer)?, current_dir);

    Ok(())
}
