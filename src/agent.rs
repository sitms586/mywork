use std::collections::HashMap;

use crate::config::AgentConfig;
use crate::logger::Logger;
use crate::tasks::{branch, pr_create, review};

pub struct OpenCodeAgent {
    config: AgentConfig,
}

impl OpenCodeAgent {
    pub fn new() -> Self {
        Self {
            config: AgentConfig::default(),
        }
    }

    #[allow(dead_code)]
    pub fn with_config(config: AgentConfig) -> Self {
        Self { config }
    }

    pub async fn execute_task(&self, task_type: &str, options: HashMap<String, String>) -> anyhow::Result<serde_json::Value> {
        Logger::info(&format!("Executing task: {}", task_type));

        match task_type {
            "create-pr" => {
                let result = self.create_pr_workflow(options).await?;
                Ok(serde_json::json!({ "success": result }))
            }
            "review-pr" => {
                let result = self.review_workflow(options).await?;
                Ok(serde_json::json!({ "success": result }))
            }
            "manage-branch" => {
                let result = self.branch_workflow(options).await?;
                Ok(serde_json::json!({ "success": result }))
            }
            _ => Err(anyhow::anyhow!("Unknown task type: {}", task_type)),
        }
    }

    async fn create_pr_workflow(&self, options: HashMap<String, String>) -> anyhow::Result<bool> {
        let branch_name = options.get("branchName").cloned().unwrap_or_default();
        let title = options.get("title").cloned().unwrap_or_default();
        let description = options.get("description").cloned().unwrap_or_default();
        let labels: Vec<String> = options
            .get("labels")
            .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
            .unwrap_or_default();

        branch::create_branch(&self.config.github, &branch_name).await?;

        let pr_created = pr_create::create_pr(
            &self.config.github,
            pr_create::PrCreateOptions {
                title,
                body: description,
                source_branch: branch_name.clone(),
                target_branch: None,
                labels: Some(labels),
            },
        )
        .await?;

        if self.config.tasks.require_review && pr_created {
            self.review_workflow(options).await?;
        }

        Ok(pr_created)
    }

    async fn review_workflow(&self, options: HashMap<String, String>) -> anyhow::Result<bool> {
        let branch_name = options.get("branchName").cloned().unwrap_or_default();
        review::review_pr(&self.config.github, &branch_name).await
    }

    async fn branch_workflow(&self, options: HashMap<String, String>) -> anyhow::Result<bool> {
        let action = options.get("action").cloned().unwrap_or_default();
        let branch_name = options.get("branchName").cloned().unwrap_or_default();

        match action.as_str() {
            "create" => branch::create_branch(&self.config.github, &branch_name).await,
            "delete" => branch::delete_branch(&self.config.github, &branch_name).await,
            _ => Err(anyhow::anyhow!("Unknown branch action: {}", action)),
        }
    }
}
