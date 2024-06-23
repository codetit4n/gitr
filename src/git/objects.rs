use crate::git::repo::GitRepository;
use crate::git::utils::repo_file;
use flate2::bufread::ZlibDecoder;
use std::fs;
use std::io;
use std::io::prelude::*;

/// Represents a git object
#[derive(Debug)]
pub struct GitObject {
    data: Option<Vec<u8>>,
}

impl GitObject {
    pub fn read(repo: GitRepository, sha: &str) -> Option<Self> {
        let (part_1, part_2) = sha.split_at(2);
        let path_to_obj = &format!("objects/{part_1}/{part_2}");
        let path = repo_file(&repo.gitdir, path_to_obj, false)?;
        if !path.exists() {
            return None;
        }
        let read_data = fs::read(path).expect("Failed to read object file for {sha}");
        let mut decompressor = ZlibDecoder::new(&read_data[..]);
        let mut raw_data: Vec<u8> = Vec::new();
        decompressor
            .read_to_end(&mut raw_data)
            .expect("Failed to decompress object file for {sha}");
        let idx_type = raw_data.iter().position(|&b| b == b' ')?;

        //size validation
        //let idx = &raw_data[idx..b'\x00'].iter().position(|&b| b == b' ')?;
        let idx_aft_type = raw_data[idx_type..].iter().position(|&b| b == b'\x00')?;
        let idx_null_byte = idx_type + idx_aft_type;
        dbg!(&raw_data[idx_type..idx_null_byte]);
        let d = String::from_utf8(Vec::from(&raw_data[idx_type..idx_null_byte]));
        dbg!(d);

        //match &raw_data[..idx] {
        //    b"blob" => dbg!("hii!"),
        //    _ => "bye!",
        //};

        todo!();
    }
}
