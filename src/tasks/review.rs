use crate::logger::Logger;

pub async fn review_pr(branch_name: &str) -> anyhow::Result<bool> {
    Logger::info(&format!("Reviewing PR for branch: {}", branch_name));

    let output = std::process::Command::new("opencode")
        .args(["pr", "review", "--branch", branch_name])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute review command: {}", e))?;

    if output.status.success() {
        Logger::success("Review completed");
        Ok(true)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Review failed: {}", stderr))
    }
}
