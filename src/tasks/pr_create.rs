use crate::config::GithubConfig;
use crate::logger::Logger;

pub struct PrCreateOptions {
    pub title: String,
    pub body: String,
    pub source_branch: String,
    pub target_branch: Option<String>,
    #[allow(dead_code)]
    pub labels: Option<Vec<String>>,
}

pub async fn create_pr(config: &GithubConfig, options: PrCreateOptions) -> anyhow::Result<bool> {
    let target_branch = options.target_branch.unwrap_or_else(|| "main".to_string());

    Logger::info(&format!("Creating PR: {}", options.title));

    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls",
        config.owner, config.repo
    );

    let body = serde_json::json!({
        "title": options.title,
        "body": options.body,
        "head": options.source_branch,
        "base": target_branch,
    });

    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "opencode-agent-rust")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&body)
        .send()
        .await?;

    if resp.status().is_success() {
        Logger::success("PR created successfully");
        Ok(true)
    } else {
        let status = resp.status();
        let text = resp.text().await?;
        Err(anyhow::anyhow!("GitHub API error ({}): {}", status, text))
    }
}
