mod agent;
mod config;
mod logger;
mod tasks;

use std::collections::HashMap;

use agent::OpenCodeAgent;
use logger::Logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let agent = OpenCodeAgent::new();

    let mut options = HashMap::new();
    options.insert("branchName".to_string(), "feature/rust-agent-pr".to_string());
    options.insert("title".to_string(), "feat: 添加自动化功能".to_string());
    options.insert("description".to_string(), "这是一个由 OpenCode Agent 自动创建的 PR".to_string());
    options.insert("labels".to_string(), "automation,enhancement".to_string());

    let result = agent.execute_task("create-pr", options).await;

    match result {
        Ok(val) => {
            Logger::success(&format!("自动化任务执行成功！{:?}", val));
            Ok(())
        }
        Err(e) => {
            Logger::error(&format!("自动化任务执行失败！{}", e));
            Err(e)
        }
    }
}
