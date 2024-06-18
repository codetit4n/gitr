use crate::git::repo::GitRepository;
use crate::git::utils::{repo_dir, repo_file, repo_path};
use serde_ini;
use std::fs;

/// Create a new git repository at the given path
pub fn create_repo(path: &str) -> GitRepository {
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

    repo_dir(&repo_path(&repo.gitdir, "branches"), true)
        .expect("Failed to create .git/branches directory");
    repo_dir(&repo_path(&repo.gitdir, "objects"), true)
        .expect("Failed to create .git/objects directory");
    repo_dir(&repo_path(&repo.gitdir, "refs/tags"), true)
        .expect("Failed to create .git/refs/tags directory");
    repo_dir(&repo_path(&repo.gitdir, "refs/heads"), true)
        .expect("Failed to create .git/refs/heads directory");

    fs::write(
        &repo_file(&repo.gitdir, "description", true)
            .expect("Failed to create .git/description file"),
        "Unnamed repository; edit this file 'description' to name the repository.\n",
    )
    .expect("Failed to write .git/description file");

    fs::write(
        &repo_file(&repo.gitdir, "HEAD", true).expect("Failed to create .git/HEAD file"),
        "ref: refs/heads/master\n",
    )
    .expect("Failed to write .git/HEAD file");

    fs::write(
        &repo_file(&repo.gitdir, "config", true).expect("Failed to create .git/config file"),
        serde_ini::to_string(&repo.config).expect("Failed to serialize GitConfig"),
    )
    .expect("Failed to write .git/config file");

    repo
}
