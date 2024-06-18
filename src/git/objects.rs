use crate::git::repo::GitRepository;
use crate::git::utils::repo_file;

/// Represents a git object
#[derive(Debug)]
pub struct GitObject<'a> {
    data: Option<&'a [u8]>,
}

impl<'a> GitObject<'a> {
    /// Create a new GitObject
    fn new(data: Option<&'a [u8]>) -> Self {
        GitObject { data }
    }

    pub fn read(repo: GitRepository, sha: &str) -> Option<Self> {
        let (part_1, part_2) = sha.split_at(2);
        let path_to_obj = &format!("objects/{part_1}/{part_2}");
        let path = repo_file(&repo.gitdir, path_to_obj, false);
        dbg!(&path);
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::repo::GitRepository;

    #[test]
    fn test_git_object_read() {
        let repo = GitRepository::new(".", false);
        let sha = "0851d920e3ca968340cb81fd2a8f6b819c76bf10";
        let obj = GitObject::read(repo, sha);
        dbg!(&obj);
        //assert!(obj.is_some());
    }
}
