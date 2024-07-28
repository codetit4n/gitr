use crate::git::objects::objects::GitObject;

#[derive(Debug)]
pub struct GitTag {
    pub fmt: Vec<u8>,
    pub data: Vec<u8>,
}

impl GitObject for GitTag {
    fn fmt(&self) -> Vec<u8> {
        self.fmt.clone()
    }
}
