use crate::types::git_repo::GitRepository;
use crate::utils::*;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

/// gitr: Git in Rust
#[derive(Parser, Debug)]
#[clap(version)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize an empty gitr repository
    ///
    Init {
        /// Where to create the repository
        path: Option<PathBuf>,
    },
    /// Add a file to the staging area
    ///
    Add,
    /// Provide content or details of repository objects
    ///
    CatFile,
    /// Debug gitignore / exclude files
    ///
    CheckIgnore,
    /// Switch branches or restore working tree files
    ///
    Checkout,
    /// Record changes to the repository
    ///
    Commit,
    /// Compute object ID and optionally create an object from a file
    ///
    HashObject,
    /// Show commit logs
    ///
    Log,
    /// Show information about files in the index and the working tree
    ///
    LsFiles,
    /// Pick out and massage parameters
    ///
    RevParse,
    /// Remove files from the working tree and from the index
    ///
    Rm,
    /// List references in a local repository
    ///
    ShowRef,
    /// Show the working tree status
    ///
    Status,
    /// Create, list, delete or verify a tag object signed with GPG
    ///
    Tag,
}

impl Commands {
    pub fn execute(&self) {
        match self {
            Commands::Init { path } => {
                let repo = if path.is_none() {
                    create_repo(".")
                } else {
                    create_repo(path.as_ref().unwrap().to_str().unwrap())
                };

                println!(
                    "Initialized empty git repository in {}",
                    repo.worktree.canonicalize().unwrap().display()
                );
            }
            _ => unimplemented!(),
        }
    }
}

fn create_repo(path: &str) -> GitRepository {
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
