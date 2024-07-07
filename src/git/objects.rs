use crate::git::repo::GitRepository;
use flate2::bufread::ZlibDecoder;
use std::fs;
use std::io::prelude::*;

/// Represents a git object
pub trait GitObject {
    fn get_data(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct GitBlob {
    data: Vec<u8>,
}

impl GitObject for GitBlob {
    fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[derive(Debug)]
pub struct GitCommit {
    data: Vec<u8>,
}

impl GitObject for GitCommit {
    fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[derive(Debug)]
pub struct GitTree {
    data: Vec<u8>,
}

impl GitObject for GitTree {
    fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[derive(Debug)]
pub struct GitTag {
    data: Vec<u8>,
}

impl GitObject for GitTag {
    fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

pub fn read_git_object(repo: GitRepository, sha: &str) -> Option<Box<dyn GitObject>> {
    //let path_to_obj = repo.gitdir.join("objects").join(&sha[0..2]).join(&sha[2..]);

    let (part_1, part_2) = sha.split_at(2);
    let path_to_obj = &format!("objects/{part_1}/{part_2}");
    let path = repo_file(&repo.gitdir, path_to_obj, false)?;
    if !path.exists() {
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

    if size != raw_data.len() - y - 1 {
        panic!("Malformed object {sha} : bad length");
    }

    match fmt {
        b"blob" => {
            return Some(Box::new(GitBlob {
                data: raw_data[y + 1..].to_vec(),
            }));
        }
        b"tree" => {
            return Some(Box::new(GitTree {
                data: raw_data[y + 1..].to_vec(),
            }));
        }
        b"commit" => {
            return Some(Box::new(GitCommit {
                data: raw_data[y + 1..].to_vec(),
            }));
        }
        b"tag" => {
            return Some(Box::new(GitTag {
                data: raw_data[y + 1..].to_vec(),
            }));
        }
        _ => panic!(
            "Unknown type {} for object {sha}",
            std::str::from_utf8(fmt).unwrap()
        ),
    }
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

        let obj = read_git_object(repo, &sha);
        assert!(obj.is_some());
    }
}
