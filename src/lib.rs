use std::path::{Path, PathBuf};

/// Represents a Git Repository
struct GitRepository {
    worktree: PathBuf,
    gitdir: PathBuf,
}

impl GitRepository {
    pub fn new(path: &str, force: bool) -> GitRepository {
        let worktree = PathBuf::from(path);
        let gitdir = worktree.join(".git");

        GitRepository { worktree, gitdir }
        /// TODO
    }
}
