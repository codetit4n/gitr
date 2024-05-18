use clap::{Parser, Subcommand};

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
    /// This command creates a new gitr repository with no commits.
    Init,
    /// Add a file to the staging area
    ///
    /// This command adds a file to the staging area.
    Add,
}
