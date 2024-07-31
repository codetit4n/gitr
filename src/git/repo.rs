use crate::git::config::GitConfig;
use std::{fs, path::Path, path::PathBuf};

/// Represents a Git Repository
#[derive(Debug, Clone)]
pub struct GitRepository {
    pub worktree: PathBuf,
    pub gitdir: PathBuf,
    pub config: GitConfig,
}

impl GitRepository {
    pub fn new(path: &str, force: bool) -> Self {
        let path = PathBuf::from(path);
        let gitdir_path = path.join(".git");
        if !(force || gitdir_path.is_dir()) {
            panic!("Not a Git Repository {}", path.display());
        } else {
            let mut repo = Self {
                worktree: path,
                gitdir: gitdir_path,
                config: GitConfig::default(),
            };
            let config_path = repo_file(&repo, "config", false);

            if config_path.is_some() {
                if config_path.clone().unwrap().exists() {
                    let read_config = fs::read_to_string(config_path.unwrap())
                        .expect("Failed to read .git/config file");
                    let config: GitConfig =
                        serde_ini::from_str(&read_config).expect("Failed to parse .git/config");
                    repo.config = config;
                }
            } else if !force {
                panic!(".git/config file missing");
            }

            if !force {
                let ver = repo
                    .config
                    .core
                    .repositoryformatversion
                    .as_ref()
                    .expect("No repositoryformatversion in .git/config");
                if ver != "0" {
                    panic!("Unsupported repositoryformatversion {}", ver);
                }
            }

            repo
        }
    }
}

pub fn repo_path(repo: &GitRepository, path: &str) -> PathBuf {
    repo.gitdir.join(path)
}

/// Return and optionally create a path to a file
pub fn repo_file(repo: &GitRepository, path: &str, mkdir: bool) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    repo_dir(
        repo,
        path.parent()
            .expect("Failed to get parent path {path.parent().display()}")
            .to_str()
            .expect("Failed to convert parent path to string"),
        mkdir,
    )
    .map(|_| {
        repo_path(
            repo,
            path.to_str().expect("Failed to convert path to string"),
        )
    })
}

/// Return and optionally create a path to a directory
pub fn repo_dir(repo: &GitRepository, path: &str, mkdir: bool) -> Option<PathBuf> {
    let path = repo_path(repo, path);
    if path.exists() {
        match path.is_dir() {
            true => return Some(PathBuf::from(path)),
            false => panic!("Not a directory {}", path.display()),
        }
    }

    if mkdir {
        fs::create_dir_all(&path).expect("Failed to create directory");
        return Some(PathBuf::from(path));
    }
    None
}

/// Find the git repository in the given path
pub fn repo_find(path: &str, required: bool) -> Option<GitRepository> {
    let path = Path::new(path);
    let path = Path::canonicalize(path).expect("Failed to canonicalize path");
    let gitdir = path.join(".git");

    if gitdir.is_dir() {
        return Some(GitRepository::new(path.to_str().unwrap(), false));
    }
    let parent = path.join("../");

    if parent == path {
        if required {
            panic!("No git directory.")
        } else {
            return None;
        }
    }

    repo_find(
        parent
            .to_str()
            .expect("Failed to convert parent path to string"),
        required,
    )
}
