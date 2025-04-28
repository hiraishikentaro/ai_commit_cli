use crate::api::ANTHROPIC_API_BASE_ENV;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
pub struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
pub struct Message {
    role: String,
    content: Vec<Content>,
}

#[derive(Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct ClaudeResponse {
    content: Vec<ResponseContent>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

pub async fn call_api(
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String> {
    let client = Client::new();

    // ベースURLを環境変数から取得（テスト用）
    let base_url = env::var(ANTHROPIC_API_BASE_ENV)
        .unwrap_or_else(|_| "https://api.anthropic.com".to_string());
    let url = format!("{}/v1/messages", base_url);

    let request = ClaudeRequest {
        model: model.to_string(),
        max_tokens: 1000,
        system: system_prompt.to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: vec![Content {
                content_type: "text".to_string(),
                text: user_prompt.to_string(),
            }],
        }],
    };

    let response = client
        .post(url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow!("Claude API error: {}", error_text));
    }

    let claude_response: ClaudeResponse = response.json().await?;

    if claude_response.content.is_empty() || claude_response.content[0].content_type != "text" {
        return Err(anyhow!("Unexpected response format from Claude API"));
    }

    Ok(claude_response.content[0].text.clone())
}
