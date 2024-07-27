use crate::git::repo::{repo_file, GitRepository};
use flate2::bufread::ZlibDecoder;
use std::fs;
use std::io::prelude::*;

/// Trait for Git objects
pub trait GitObject {
    fn serialize(&self, repo: &GitRepository) {
        unimplemented!()
    }
    fn deserialize(&mut self, data: &[u8]) {
        unimplemented!()
    }
    fn init(&mut self) {}
}

struct GitCommit {
    data: Vec<u8>,
}

struct GitTree {
    data: Vec<u8>,
}

struct GitTag {
    data: Vec<u8>,
}

struct GitBlob {
    data: Vec<u8>,
}

impl GitObject for GitCommit {}
impl GitObject for GitTree {}
impl GitObject for GitTag {}
impl GitObject for GitBlob {}

fn object_read(repo: &GitRepository, sha: &str) -> Option<Box<dyn GitObject>> {
    let path = repo_file(
        &repo,
        &format!("objects/{}/{}", &sha[0..2], &sha[2..]),
        false,
    )?;

    if !path.is_file() {
        return None;
    }

    let read_data = fs::read(path).expect("Failed to read object file for {sha}");
    let mut decompressor = ZlibDecoder::new(&read_data[..]);
    let mut decompressed_data: Vec<u8> = Vec::new();
    decompressor
        .read_to_end(&mut decompressed_data)
        .expect("Failed to decompress object file for {sha}");
    let raw_data = decompressed_data;
    let x = raw_data.iter().position(|&b| b == b' ')?;
    let y = raw_data.iter().position(|&b| b == b'\x00')?;
    let fmt = &raw_data[0..x];
    let size_str = &raw_data[x + 1..y];
    let size: usize = std::str::from_utf8(size_str)
        .expect("Malformed object {sha} : bad size")
        .parse()
        .expect("Malformed object {sha} : bad size");

    if size != raw_data[..].len() - y - 1 {
        panic!("Malformed object {sha}: bad length")
    }

    let obj: Box<dyn GitObject> = match fmt {
        b"commit" => Box::new(GitCommit {
            data: raw_data[y + 1..].to_vec(),
        }),
        b"tree" => Box::new(GitTree {
            data: raw_data[y + 1..].to_vec(),
        }),
        b"tag" => Box::new(GitTag {
            data: raw_data[y + 1..].to_vec(),
        }),
        b"blob" => Box::new(GitBlob {
            data: raw_data[y + 1..].to_vec(),
        }),
        _ => {
            let fmt_str = std::str::from_utf8(fmt).unwrap();
            panic!("Unknown type {fmt_str} for object {sha}")
        }
    };

    Some(obj)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_read_git_object() {
        let repo = GitRepository::new(".", true);

        // read the current git repo
        let mut paths = fs::read_dir("./.git/objects").unwrap();
        //get the first object
        let path = paths.next().unwrap().unwrap().path();
        let path_first_obj = fs::read_dir(path).unwrap().next().unwrap().unwrap().path();
        let sha = String::from(
            path_first_obj
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        ) + path_first_obj.file_name().unwrap().to_str().unwrap();

        let obj = object_read(&repo, &sha);
        assert!(obj.is_some());
    }
}
