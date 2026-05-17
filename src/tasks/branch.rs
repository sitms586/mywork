use crate::logger::Logger;

pub async fn create_branch(branch_name: &str) -> anyhow::Result<bool> {
    Logger::info(&format!("Creating branch: {}", branch_name));

    let output = std::process::Command::new("git")
        .args(["checkout", "-b", branch_name])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to create branch: {}", e))?;

    if output.status.success() {
        Logger::success(&format!("Branch '{}' created", branch_name));
        Ok(true)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Branch creation failed: {}", stderr))
    }
}

pub async fn delete_branch(branch_name: &str) -> anyhow::Result<bool> {
    Logger::info(&format!("Deleting branch: {}", branch_name));

    let output = std::process::Command::new("git")
        .args(["branch", "-d", branch_name])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to delete branch: {}", e))?;

    if output.status.success() {
        Logger::success(&format!("Branch '{}' deleted", branch_name));
        Ok(true)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Branch deletion failed: {}", stderr))
    }
}
