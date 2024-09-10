use crate::git::objects::kvlm::Dict;
use crate::git::objects::kvlm::{kvlm_parse, kvlm_serialize};
use crate::git::objects::objects::GitObject;
use crate::git::repo::GitRepository;
use std::any::Any;

#[derive(Debug)]
pub struct GitCommit {
    pub fmt: Vec<u8>,
    pub kvlm: Dict,
}

impl GitObject for GitCommit {
    fn fmt(&self) -> Vec<u8> {
        self.fmt.clone()
    }

    fn deserialize(&mut self, data: Vec<u8>) {
        self.kvlm = kvlm_parse(&data, 0, None);
    }

    fn serialize(&self, _: Option<GitRepository>) -> Vec<u8> {
        return kvlm_serialize(self.kvlm.clone());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
