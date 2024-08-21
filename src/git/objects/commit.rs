use crate::git::objects::objects::GitObject;

#[derive(Debug)]
pub struct GitCommit {
    pub fmt: Vec<u8>,
    pub data: Vec<u8>,
}

impl GitObject for GitCommit {
    fn fmt(&self) -> Vec<u8> {
        self.fmt.clone()
    }
}
