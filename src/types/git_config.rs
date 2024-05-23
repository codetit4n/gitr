use serde::Deserialize;

/// Represents a Git Config file located in the .git directory
#[derive(Debug, Deserialize)]
pub struct GitConfig {
    pub core: Core,
}

#[derive(Debug, Deserialize)]
pub struct Core {
    pub repositoryformatversion: Option<String>,
    pub filemode: Option<String>,
    pub bare: Option<String>,
    pub logallrefupdates: Option<String>,
    pub symlinks: Option<String>,
    pub ignorecase: Option<String>,
}

impl Default for GitConfig {
    fn default() -> Self {
        GitConfig {
            core: Core {
                repositoryformatversion: Some("0".to_string()),
                filemode: Some("false".to_string()),
                bare: Some("false".to_string()),
                logallrefupdates: None,
                symlinks: None,
                ignorecase: None,
            },
        }
    }
}
