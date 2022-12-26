use crate::parser::Log;

#[derive(Debug)]
pub struct TreeNodes {
    name: String,
    is_file: bool,
    next: Vec<TreeNodes>
}

pub fn build(logs: Vec<Log>, current_dir: String) -> TreeNodes
{
    let mut tree: TreeNodes = TreeNodes { name: current_dir, is_file: false, next: vec![] };

    for log in logs {
        let path: Vec<String> = log.get_path();
        add_file_or_folder(&mut tree, path);
    }

    tree
}

fn add_file_or_folder(tree: &mut TreeNodes, mut path: Vec<String>)
{
    if path.len() == 1 {
        for node in &tree.next {
            if node.name == path[0] && node.is_file {
                return
            }
        }
        tree.next.push(TreeNodes {
            name: path[0].clone(),
            is_file: true,
            next: vec![],
        });
    } else if path.len() > 1 {
        let mut index: usize = 0;
        while index < tree.next.len() {
            if tree.next[index].name == path[0] && !tree.next[index].is_file {
                path.remove(0);
                return add_file_or_folder(&mut tree.next[index], path);
            }
            index += 1;
        }
        tree.next.push(TreeNodes {
            name: path.remove(0),
            is_file: false,
            next: vec![],
        });
        add_file_or_folder(&mut tree.next[index], path);
    }
}
