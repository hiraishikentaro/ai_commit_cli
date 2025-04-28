pub mod claude;
pub mod gemini;
pub mod openai;

use crate::config::Platform;
use anyhow::Result;

// ベースURLの環境変数名を定義（テスト用）
pub const ANTHROPIC_API_BASE_ENV: &str = "ANTHROPIC_API_BASE";
pub const OPENAI_API_BASE_ENV: &str = "OPENAI_API_BASE";
pub const GEMINI_API_BASE_ENV: &str = "GEMINI_API_BASE";

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
