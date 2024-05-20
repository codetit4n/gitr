use std::fs;
use std::path::{Path, PathBuf};

/// Compute path under repo's gitdir
pub fn repo_path(gitdir: &Path, path: &str) -> PathBuf {
    Path::new(gitdir).join(path)
}

/// Return and optionally create a path to a file
pub fn repo_file(gitdir: &Path, path: &str, mkdir: bool) -> Option<PathBuf> {
    let path = repo_path(gitdir, path);
    repo_dir(
        path.parent()
            .expect("Failed to get parent path {path.parent().display()}"),
        mkdir,
    )
    .map(|_| path)
}

/// Return and optionally create a path to a directory
pub fn repo_dir(path: &Path, mkdir: bool) -> Option<PathBuf> {
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
