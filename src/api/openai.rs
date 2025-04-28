use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OpenAIRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<OpenAIMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIChoice {
    message: OpenAIMessage,
}

pub async fn call_api(
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String> {
    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";

    let request = OpenAIRequest {
        model: model.to_string(),
        max_tokens: 1000,
        messages: vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            OpenAIMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ],
    };

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow!("OpenAI API error: {}", error_text));
    }

    let openai_response: OpenAIResponse = response.json().await?;

    if openai_response.choices.is_empty() {
        return Err(anyhow!("Unexpected response format from OpenAI API"));
    }

    Ok(openai_response.choices[0].message.content.clone())
}
