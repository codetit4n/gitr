use crate::git::objects::objects::GitObject;
use std::any::Any;
use std::path::PathBuf;

#[derive(Debug)]
pub struct GitTree {
    pub fmt: Vec<u8>,
    pub data: Vec<u8>,
}

impl GitObject for GitTree {
    fn fmt(&self) -> Vec<u8> {
        self.fmt.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct GitTreeLeaf {
    pub mode: Vec<u8>,
    pub path: PathBuf,
    pub sha: String,
}

impl GitTreeLeaf {
    fn new(mode: Vec<u8>, path: PathBuf, sha: String) -> Self {
        Self { mode, path, sha }
    }
}

fn tree_parse_one(raw: Vec<u8>, start: usize) -> (usize, GitTreeLeaf) {
    // Find the space terminator of the mode

    let x = raw[start..].iter().position(|&x| x == b' ').unwrap_or(0);
    assert!(x - start == 5 || x - start == 6);

    let mut mode = raw[start..x].to_vec();

    if mode.len() == 5 {
        // Normalize to six bytes
        mode = [b' '].iter().chain(mode.iter()).cloned().collect();
    }

    // Find the NULl terminator of the path
    let y = raw[x..]
        .iter()
        .position(|&x| x == b'\x00')
        .expect("Invalid tree object");

    let path = PathBuf::from(std::str::from_utf8(&raw[x + 1..x + y]).unwrap());

    // Read the SHA and convert to a hex string
    let slice = &raw[y + 1..y + 21];
    let mut int_value: u128 = 0;
    for &byte in slice {
        int_value = (int_value << 8) | byte as u128;
    }
    let sha = format!("{:040x}", int_value);

    (y + 21, GitTreeLeaf::new(mode, path, sha))
}

fn tree_parse(raw: Vec<u8>) -> Vec<GitTreeLeaf> {
    let mut pos = 0;
    let max = raw.len();
    let mut ret: Vec<GitTreeLeaf> = Vec::new();
    let mut data;
    while pos < max {
        (pos, data) = tree_parse_one(raw.clone(), pos);
        ret.push(data);
    }
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_pare() {
        let raw = vec![
            0x31, 0x30, 0x30, 0x36, 0x34, 0x34, 0x20, 0x2e, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e,
            0x6f, 0x72, 0x65, 0x2e, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x67, 0x69, 0x74, 0x69,
            0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x72, 0x73, 0x74, 0x0, 0x1f, 0x6f, 0x6b, 0x2c,
            0x20, 0x2e, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x66, 0x69,
            0x6c, 0x65, 0x73, 0x2f, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e,
            0x72, 0x73, 0x74, 0x0, 0x1f, 0x6f, 0x6b, 0x2c, 0x20, 0x2e, 0x67, 0x69, 0x74, 0x69,
            0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x67, 0x69,
            0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x72, 0x73, 0x74, 0x0, 0x1f, 0x6f,
            0x6b, 0x2c, 0x20, 0x2e, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e,
            0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72,
            0x65, 0x2e, 0x72, 0x73, 0x74, 0x0, 0x1f, 0x6f, 0x6b, 0x2c, 0x20, 0x2e, 0x67, 0x69,
            0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f,
            0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x72, 0x73, 0x74, 0x0,
            0x1f, 0x6f, 0x6b, 0x2c, 0x20, 0x2e, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72,
            0x65, 0x2e, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e,
            0x6f, 0x72, 0x65, 0x2e, 0x72, 0x73, 0x74, 0x0, 0x1f, 0x6f, 0x6b, 0x2c, 0x20, 0x2e,
            0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x66, 0x69, 0x6c, 0x65,
            0x73, 0x2f, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x72, 0x73,
            0x74, 0x0, 0x1f, 0x6f, 0x6b, 0x2c, 0x20, 0x2e, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e,
            0x6f, 0x72, 0x65, 0x2e, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x67, 0x69, 0x74, 0x69,
            0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x72, 0x73, 0x74, 0x0, 0x1f, 0x6f, 0x6b, 0x2c,
            0x20, 0x2e, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x66, 0x69,
            0x6c, 0x65, 0x73, 0x2f, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e,
            0x72, 0x73, 0x74, 0x0, 0x1f, 0x6f, 0x6b, 0x2c, 0x20, 0x2e, 0x67, 0x69, 0x74, 0x69,
            0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x67, 0x69,
            0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x72, 0x73, 0x74, 0x0, 0x1f, 0x6f,
            0x6b, 0x2c, 0x20, 0x2e, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e,
            0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72,
            0x65, 0x2e, 0x72, 0x73, 0x74, 0x0, 0x1f, 0x6f, 0x6b, 0x2c, 0x20, 0x2e, 0x67, 0x69,
            0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f,
            0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x72, 0x73, 0x74, 0x0,
            0x1f, 0x6f, 0x6b, 0x2c, 0x20, 0x2e, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72,
            0x65, 0x2e, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e,
            0x6f, 0x72, 0x65, 0x2e, 0x72, 0x73, 0x74, 0x0, 0x1f, 0x6f, 0x6b, 0x2c, 0x20, 0x2e,
            0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x66, 0x69, 0x6c, 0x65,
            0x73, 0x2f, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x2e, 0x72, 0x73,
            0x74, 0x0, 0x1f, 0x6f, 0x6b, 0x2c, 0x20, 0x2e, 0x67, 0x69, 0x74, 0x69, 0x67, 0x6e,
        ];

        let tree = tree_parse(raw);
        assert_eq!(tree.len(), 100);
        assert_eq!(tree[0].mode, vec![b'1', b'0', b'0', b'6', b'4', b'4']);
        assert_eq!(
            tree[0].path,
            PathBuf::from(".gitignore.files/gitignore.rst")
        );
        assert_eq!(
            tree[0].sha,
            "1f6f6b2c20e2e67697469676e6f72652e66696c65732f67697469676e6f72652e727374".to_string()
        );
        // assert_eq!(tree[0].mode, vec![b'1', b'0', b'0', b'6', b'4', b'4']);
        // assert_eq!(tree[0].path, PathBuf::from(".gitignore.files/gitignore.rst"));
    }
}
