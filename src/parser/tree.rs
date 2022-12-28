use super::logs;

enum TreeNode {
    FolderNode(Folder),
    FileNode(logs::Log),
    EmptyNode
}

struct Folder {
    name: String,
    childs: Vec<TreeNode>
}
