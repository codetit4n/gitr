use serde::Deserialize;
use std::{fs, path::PathBuf};

pub mod cli;
mod utils;

/// Represents a Git Config file located in the .git directory
#[derive(Debug, Deserialize)]
pub struct GitConfig {
    pub core: Core,
}

#[derive(Debug, Deserialize)]
pub struct Core {
    pub repositoryformatversion: Option<String>,
    pub filemode: Option<String>,
    pub bare: Option<String>,
    pub logallrefupdates: Option<String>,
    pub symlinks: Option<String>,
    pub ignorecase: Option<String>,
}

/// Represents a Git Repository
#[derive(Debug)]
pub struct GitRepository {
    pub worktree: PathBuf,
    pub gitdir: PathBuf,
    pub config: GitConfig,
}

impl GitRepository {
    pub fn new(path: &str, force: bool) -> Self {
        let gitdir_path = PathBuf::from(path).join(".git");
        if !(force || gitdir_path.is_dir()) {
            panic!("Not a Git Repository {}", path);
        } else {
            use utils::repo_file;
            let config_path = repo_file(&gitdir_path, "config", false);

            let mut read_config = String::new();
            if config_path.is_some() {
                read_config = fs::read_to_string(config_path.unwrap())
                    .expect("Failed to read .git/config file");
            } else if !force {
                panic!(".git/config file missing");
            }

            let config: GitConfig =
                serde_ini::from_str(&read_config).expect("Failed to parse .git/config");
            if !force {
                let ver = config
                    .core
                    .repositoryformatversion
                    .as_ref()
                    .expect("No repositoryformatversion in .git/config");
                if ver != "0" {
                    panic!("Unsupported repositoryformatversion {}", ver);
                }
            }

            GitRepository {
                worktree: PathBuf::from(path),
                gitdir: gitdir_path,
                config,
            }
        }
    }
}
