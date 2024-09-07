use crate::git::objects::objects::GitObject;
use std::any::Any;

#[derive(Debug)]
pub struct GitTag {
    pub fmt: Vec<u8>,
    pub data: Vec<u8>,
}

impl GitObject for GitTag {
    fn fmt(&self) -> Vec<u8> {
        self.fmt.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
