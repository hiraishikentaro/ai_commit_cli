pub mod claude;
pub mod gemini;
pub mod openai;

use crate::config::Platform;
use anyhow::Result;

pub async fn generate_commit_message(
    platform: Platform,
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String> {
    match platform {
        Platform::Claude => claude::call_api(api_key, model, system_prompt, user_prompt).await,
        Platform::OpenAI => openai::call_api(api_key, model, system_prompt, user_prompt).await,
        Platform::Gemini => gemini::call_api(api_key, model, system_prompt, user_prompt).await,
    }
}
