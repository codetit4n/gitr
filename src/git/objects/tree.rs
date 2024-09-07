use crate::git::objects::objects::GitObject;
use std::any::Any;

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
