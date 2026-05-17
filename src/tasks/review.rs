use crate::config::GithubConfig;
use crate::logger::Logger;

pub async fn review_pr(config: &GithubConfig, branch_name: &str) -> anyhow::Result<bool> {
    Logger::info(&format!("Reviewing PR for branch: {}", branch_name));

    let client = reqwest::Client::new();
    let search_url = format!(
        "https://api.github.com/search/issues?q=repo:{}/{}+head:{}+type:pr+is:open",
        config.owner, config.repo, branch_name
    );

    let search_resp = client
        .get(&search_url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "opencode-agent-rust")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    let search_data: serde_json::Value = search_resp.json().await?;
    let items = search_data["items"].as_array().ok_or_else(|| anyhow::anyhow!("Invalid search response"))?;

    if items.is_empty() {
        return Err(anyhow::anyhow!("No open PR found for branch: {}", branch_name));
    }

    let pr_number = items[0]["number"].as_u64().ok_or_else(|| anyhow::anyhow!("Missing PR number"))?;

    let review_url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}/reviews",
        config.owner, config.repo, pr_number
    );

    let review_body = serde_json::json!({
        "body": "Auto-review by OpenCode Agent",
        "event": "COMMENT",
    });

    let review_resp = client
        .post(&review_url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "opencode-agent-rust")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&review_body)
        .send()
        .await?;

    if review_resp.status().is_success() {
        Logger::success(&format!("Review submitted for PR #{}", pr_number));
        Ok(true)
    } else {
        let status = review_resp.status();
        let text = review_resp.text().await?;
        Err(anyhow::anyhow!("Review API error ({}): {}", status, text))
    }
}
