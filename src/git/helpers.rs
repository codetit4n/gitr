use crate::git::repo::{repo_dir, repo_file, GitRepository};
use serde_ini;
use std::fs;

/// Create a new git repository at the given path
pub fn repo_create(path: &str) -> GitRepository {
    let repo = GitRepository::new(path, true);

    if repo.worktree.exists() {
        if !repo.worktree.is_dir() {
            panic!("Not a directory {}", path);
        }
        if repo.gitdir.exists()
            && fs::read_dir(&repo.gitdir)
                .expect("Failed to read .git directory")
                .into_iter()
                .nth(0)
                .is_some()
        {
            panic!("{} is not empty", repo.gitdir.display());
        }
    } else {
        fs::create_dir_all(&repo.worktree).expect("Failed to create worktree directory");
    }

    assert!(repo_dir(&repo, "branches", true).is_some());
    assert!(repo_dir(&repo, "objects", true).is_some());
    assert!(repo_dir(&repo, "refs/tags", true).is_some());
    assert!(repo_dir(&repo, "refs/heads", true).is_some());

    fs::write(
        repo_file(&repo, "description", false).expect("Failed to get .git/description file"),
        "Unnamed repository; edit this file 'description' to name the repository.\n",
    )
    .expect("Failed to write .git/description file");

    fs::write(
        repo_file(&repo, "HEAD", false).expect("Failed to get .git/HEAD file"),
        "ref: refs/heads/master\n",
    )
    .expect("Failed to write .git/HEAD file");

    fs::write(
        repo_file(&repo, "config", false).expect("Failed to get .git/config file"),
        serde_ini::to_string(&repo.config).expect("Failed to serialize GitConfig"),
    )
    .expect("Failed to write .git/config file");

    repo
}
