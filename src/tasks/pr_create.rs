use std::process::Command;

use crate::logger::Logger;

pub struct PrCreateOptions {
    pub title: String,
    pub body: String,
    pub source_branch: String,
    pub target_branch: Option<String>,
    pub labels: Option<Vec<String>>,
}

pub async fn create_pr(options: PrCreateOptions) -> anyhow::Result<bool> {
    let target_branch = options.target_branch.unwrap_or_else(|| "main".to_string());
    let labels = options.labels.unwrap_or_default();

    Logger::info(&format!("Creating PR: {}", options.title));

    let mut args = vec![
        "pr".to_string(),
        "create".to_string(),
        "--title".to_string(),
        options.title,
        "--body".to_string(),
        options.body,
        "--source".to_string(),
        options.source_branch,
        "--target".to_string(),
        target_branch,
    ];

    if !labels.is_empty() {
        args.push("--labels".to_string());
        args.push(labels.join(","));
    }

    let output = Command::new("opencode")
        .args(&args)
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute opencode command: {}", e))?;

    if output.status.success() {
        Logger::success("PR created successfully");
        Ok(true)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Command failed: {}", stderr))
    }
}
