use crate::git::helpers::create_repo;
use crate::git::objects::GitObject;
use clap::{Parser, Subcommand};
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
            Commands::CatFile => {
                println!("CatFile");
                let repo = crate::git::repo::GitRepository::new(".", false);
                GitObject::read(repo, "0851d920e3ca968340cb81fd2a8f6b819c76bf10");
                todo!();
            }
            _ => unimplemented!(),
        }
    }
}
