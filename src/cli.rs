use crate::git::{
    helpers::{cat_file, repo_create},
    repo::repo_find,
};
use clap::{Parser, Subcommand, ValueEnum};
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
    /// Initialize an empty git repository
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
    CatFile {
        /// Specify the type
        #[arg(value_enum)]
        type_: ObjectType,
        /// The object to display
        object: String,
    },
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

#[derive(Debug, Clone, ValueEnum)]
pub enum ObjectType {
    Blob,
    Commit,
    Tag,
    Tree,
}

impl Commands {
    pub fn execute(&self) {
        match self {
            Commands::Init { path } => {
                let repo = if path.is_none() {
                    repo_create(".")
                } else {
                    repo_create(path.as_ref().unwrap().to_str().unwrap())
                };

                println!(
                    "Initialized empty git repository in {}",
                    repo.worktree.canonicalize().unwrap().display()
                );
            }
            Commands::CatFile { type_, object } => {
                let repo = repo_find(".", true).expect("Not a git repository");

                cat_file(&repo, object, Some(type_.clone()));
            }
            _ => unimplemented!(),
        }
    }
}
