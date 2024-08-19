use crate::cli::ObjectType;
use crate::git::objects::objects::{object_read, object_write};
use crate::git::{
    objects::{GitBlob, GitObject},
    repo::{repo_dir, repo_file, repo_find, GitRepository},
};
use serde_ini;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;

/// Create a new git repository at the given path
pub fn cmd_repo_create(path: &str) -> GitRepository {
    let repo = GitRepository::new(path, true);

    if repo.worktree.exists() {
        if !repo.worktree.is_dir() {
            panic!("Not a directory {}", path);
        }
        if repo.gitdir.exists()
            && fs::read_dir(&repo.gitdir)
                .expect("Failed to read .git directory")
                .into_iter()
                .next()
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

pub fn cmd_cat_file(repo: GitRepository, obj: &str, fmt: Option<ObjectType>) {
    let obj = object_read(&repo, &object_find(&repo, obj, fmt, true)).unwrap();
    print!(
        "{}",
        std::str::from_utf8(&obj.serialize(Some(repo)))
            .unwrap()
            .to_string()
    );
}

fn object_find(_: &GitRepository, name: &str, _: Option<ObjectType>, _: bool) -> String {
    name.to_string()
}

pub fn cmd_hash_object(type_: &ObjectType, write: bool, path: &Path) {
    let mut repo: Option<GitRepository> = None;
    if write {
        repo = repo_find(".", true);
    }

    let fd = File::open(path).expect("Failed to open file {path}");
    let sha = object_hash(fd, type_, repo);
    println!("{sha}")
}

fn object_hash(fd: File, fmt: &ObjectType, repo: Option<GitRepository>) -> String {
    let mut data: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(fd);
    reader.read_to_end(&mut data).expect("Failed to read file");

    let obj: Box<dyn GitObject> = match fmt {
        ObjectType::Blob => Box::new(GitBlob {
            fmt: fmt.as_bytes(),
            blobdata: data,
        }),
        _ => panic!("Unknown type {}!", { fmt.to_string() }),
    };

    object_write(obj, repo)
}
