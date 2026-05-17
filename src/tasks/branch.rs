use crate::config::GithubConfig;
use crate::logger::Logger;

pub async fn create_branch(_config: &GithubConfig, branch_name: &str) -> anyhow::Result<bool> {
    Logger::info(&format!("Creating local branch: {}", branch_name));

    let output = std::process::Command::new("git")
        .args(["checkout", "-b", branch_name])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to create local branch: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Branch creation failed: {}", stderr));
    }

    Logger::success(&format!("Local branch '{}' created", branch_name));

    let push_output = std::process::Command::new("git")
        .args(["push", "-u", "origin", branch_name])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to push branch: {}", e))?;

    if push_output.status.success() {
        Logger::success(&format!("Branch '{}' pushed to remote", branch_name));
        Ok(true)
    } else {
        let stderr = String::from_utf8_lossy(&push_output.stderr);
        Err(anyhow::anyhow!("Branch push failed: {}", stderr))
    }
}

pub async fn delete_branch(config: &GithubConfig, branch_name: &str) -> anyhow::Result<bool> {
    Logger::info(&format!("Deleting remote branch: {}", branch_name));

    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/git/refs/heads/{}",
        config.owner, config.repo, branch_name
    );

    let resp = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "opencode-agent-rust")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    if resp.status().is_success() || resp.status().as_u16() == 422 {
        Logger::success(&format!("Remote branch '{}' deleted", branch_name));

        std::process::Command::new("git")
            .args(["branch", "-d", branch_name])
            .output()
            .ok();

        Ok(true)
    } else {
        let status = resp.status();
        let text = resp.text().await?;
        Err(anyhow::anyhow!("Delete branch API error ({}): {}", status, text))
    }
}
