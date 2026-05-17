use std::env;

#[allow(dead_code)]
pub struct GithubConfig {
    pub owner: String,
    pub repo: String,
    pub default_branch: String,
    pub token: String,
}

#[allow(dead_code)]
pub struct OpencodeConfig {
    pub timeout: u64,
    pub retries: u32,
}

#[allow(dead_code)]
pub struct TaskConfig {
    pub auto_merge: bool,
    pub require_review: bool,
}

#[allow(dead_code)]
pub struct AgentConfig {
    pub github: GithubConfig,
    pub opencode: OpencodeConfig,
    pub tasks: TaskConfig,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            github: GithubConfig {
                owner: "your-username".to_string(),
                repo: "your-repo".to_string(),
                default_branch: "main".to_string(),
                token: env::var("GITHUB_TOKEN").unwrap_or_default(),
            },
            opencode: OpencodeConfig {
                timeout: 30000,
                retries: 3,
            },
            tasks: TaskConfig {
                auto_merge: false,
                require_review: true,
            },
        }
    }
}
