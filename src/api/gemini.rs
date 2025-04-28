use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GeminiRequest {
    contents: Vec<GeminiContent>,
    generation_config: GeminigenerationConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeminiContent {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeminiPart {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeminigenerationConfig {
    max_output_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Deserialize, Debug)]
pub struct GeminiCandidate {
    content: GeminiContent,
}

pub async fn call_api(
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String> {
    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    // Geminiはシステムプロンプトとユーザープロンプトを結合する
    let combined_prompt = format!("{}\n\n{}", system_prompt, user_prompt);

    let request = GeminiRequest {
        contents: vec![GeminiContent {
            role: "user".to_string(),
            parts: vec![GeminiPart {
                text: combined_prompt,
            }],
        }],
        generation_config: GeminigenerationConfig {
            max_output_tokens: 1000,
        },
    };

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow!("Gemini API error: {}", error_text));
    }

    let gemini_response: GeminiResponse = response.json().await?;

    if gemini_response.candidates.is_empty() {
        return Err(anyhow!("Unexpected response format from Gemini API"));
    }

    let content = &gemini_response.candidates[0].content;
    if content.parts.is_empty() {
        return Err(anyhow!("No text in response from Gemini API"));
    }

    Ok(content.parts[0].text.clone())
}
