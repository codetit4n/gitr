use crate::git::config::GitConfig;
use std::{fs, path::Path, path::PathBuf};

/// Represents a Git Repository
#[derive(Debug)]
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
            panic!(
                "Not a Git Repository {}",
                path.canonicalize().unwrap().to_str().unwrap()
            );
        } else {
            let path = PathBuf::from(path);
            let repo = Self {
                worktree: path.clone(),
                gitdir: path.join(".git"),
                config: GitConfig::default(),
            };

            let config_path = repo.repo_file("config", false);

            let mut read_config = String::new();
            if config_path.is_some() {
                read_config = fs::read_to_string(config_path.unwrap())
                    .expect("Failed to read .git/config file");
            } else if !force {
                panic!(".git/config file missing");
            }

            let mut config: GitConfig = GitConfig::default();

            if !force {
                config = serde_ini::from_str(&read_config).expect("Failed to parse .git/config");
                let ver = config
                    .core
                    .repositoryformatversion
                    .as_ref()
                    .expect("No repositoryformatversion in .git/config");
                if ver != "0" {
                    panic!("Unsupported repositoryformatversion {}", ver);
                }
            }

            Self {
                worktree: PathBuf::from(path),
                gitdir: gitdir_path,
                config,
            }
        }
    }

    /// Compute path under repo's gitdir
    pub fn repo_path(&self, path: &str) -> PathBuf {
        Path::new(&self.gitdir).join(path)
    }

    /// Return and optionally create a path to a file
    pub fn repo_file(&self, path: &str, mkdir: bool) -> Option<PathBuf> {
        let path = self.repo_path(path);

        self.repo_dir(
            path.parent()
                .expect("Failed to get parent path {path.parent().display()}")
                .to_str()
                .unwrap(),
            mkdir,
        )
        .map(|_| path)
    }

    /// Return and optionally create a path to a directory
    pub fn repo_dir(&self, path: &str, mkdir: bool) -> Option<PathBuf> {
        let path = self.repo_path(path);

        if path.exists() {
            match path.is_dir() {
                true => return Some(path),
                false => panic!("Not a directory {}", path.display()),
            }
        }

        if mkdir {
            fs::create_dir_all(&path).expect("Failed to create directory");
            return Some(path);
        }
        None
    }
}

/// Find the git repository in the given path
pub fn repo_find(path: &Path, required: bool) -> Option<GitRepository> {
    let path = Path::canonicalize(path).expect("Failed to canonicalize path");

    if path.join(".git").is_dir() {
        return Some(GitRepository::new(path.to_str().unwrap(), false));
    } else {
        let parent = Path::canonicalize(&path.join("..")).unwrap();

        if parent == path {
            if required {
                panic!("No git directory");
            } else {
                return None;
            }
        } else {
            return repo_find(&parent, required);
        }
    }
}
