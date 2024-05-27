use serde::{Deserialize, Serialize};

/// Represents a Git Config file located in the .git directory
#[derive(Debug, Deserialize, Serialize)]
pub struct GitConfig {
    pub core: Core,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Core {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositoryformatversion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filemode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bare: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logallrefupdates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symlinks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
