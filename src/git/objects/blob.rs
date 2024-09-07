use crate::git::{objects::GitObject, repo::GitRepository};
use std::any::Any;

#[derive(Debug)]
pub struct GitBlob {
    pub fmt: Vec<u8>,
    pub blobdata: Vec<u8>,
}

impl GitObject for GitBlob {
    fn fmt(&self) -> Vec<u8> {
        self.fmt.clone()
    }

    fn serialize(&self, _: Option<GitRepository>) -> Vec<u8> {
        self.blobdata.clone()
    }

    fn deserialize(&mut self, data: Vec<u8>) {
        self.blobdata = data
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
