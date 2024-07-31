use crate::git::{
    objects::{GitBlob, GitCommit, GitTag, GitTree},
    repo::{repo_file, GitRepository},
};
use flate2::{bufread::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{
    fs,
    io::{prelude::*, BufWriter, Write},
};

/// Trait for Git objects
pub trait GitObject {
    fn fmt(&self) -> Vec<u8>;

    fn serialize(&self, _: Option<GitRepository>) -> Vec<u8> {
        unimplemented!()
    }

    fn deserialize(&mut self, _: Vec<u8>) {
        unimplemented!()
    }
    fn init(&mut self) {}
}

pub fn object_read(repo: &GitRepository, sha: &str) -> Option<Box<dyn GitObject>> {
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
        b"commit" => {
            dbg!("commit");
            Box::new(GitCommit {
                fmt: b"commit".to_vec(),
                data: raw_data[y + 1..].to_vec(),
            })
        }
        b"tree" => {
            dbg!("tree");
            Box::new(GitTree {
                fmt: b"tree".to_vec(),
                data: raw_data[y + 1..].to_vec(),
            })
        }
        b"tag" => {
            dbg!("tag");
            Box::new(GitTag {
                fmt: b"tag".to_vec(),
                data: raw_data[y + 1..].to_vec(),
            })
        }
        b"blob" => {
            dbg!("blob");
            Box::new(GitBlob {
                fmt: b"blob".to_vec(),
                blobdata: raw_data[y + 1..].to_vec(),
            })
        }
        _ => {
            let fmt_str = std::str::from_utf8(fmt).unwrap();
            panic!("Unknown type {fmt_str} for object {sha}")
        }
    };

    Some(obj)
}

pub fn object_write(object: Box<dyn GitObject>, repo: Option<GitRepository>) -> String {
    // serialize object data
    let data = object.serialize(repo.clone());

    // Add header
    let mut result = object.fmt();
    result.push(b' ');
    result.extend(data.len().to_string().as_bytes());
    result.push(b'\x00');
    result.extend(data);

    // compute sha1
    let mut hasher = Sha1::new();
    hasher.update(&result);
    let sha = hasher.finalize();
    let sha = format!("{:x}", sha);

    if repo.is_some() {
        let path = repo_file(
            &repo.unwrap(),
            &format!("objects/{}/{}", &sha[0..2], &sha[2..]),
            true,
        )
        .unwrap();

        if !path.exists() {
            // compress and write
            let file = fs::File::create(path).expect("Failed to create object file");
            let writer = BufWriter::new(file);
            let mut encoder = ZlibEncoder::new(writer, Compression::default());
            encoder
                .write_all(&result)
                .expect("Failed to write object file");
            encoder.finish().expect("Failed to write object file");
        }
    }

    sha
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_read_git_object_success() {
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

        dbg!(&sha);
        let obj = object_read(&repo, &sha);
        assert!(obj.is_some());
    }

    #[test]
    fn test_read_git_object_failure() {
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

        let sha = sha + "123";

        let obj = object_read(&repo, &sha);
        assert!(obj.is_none());
    }

    #[test]
    fn test_write_git_object() {
        let obj = Box::new(GitBlob {
            fmt: b"blob".to_vec(),
            blobdata: b"Hello World".to_vec(),
        });

        let result = object_write(obj, None);
        assert_eq!(result.len(), 40);
    }
}
