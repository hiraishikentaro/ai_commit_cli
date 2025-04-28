use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Platform {
    Claude,
    OpenAI,
    Gemini,
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Claude => "Claude",
            Platform::OpenAI => "OpenAI",
            Platform::Gemini => "Gemini",
        }
    }

    pub fn env_var_name(&self) -> &'static str {
        match self {
            Platform::Claude => "CLAUDE_API_KEY",
            Platform::OpenAI => "OPENAI_API_KEY",
            Platform::Gemini => "GEMINI_API_KEY",
        }
    }

    pub fn model_name(&self) -> &'static str {
        match self {
            Platform::Claude => "claude-3-opus-20240229",
            Platform::OpenAI => "gpt-4",
            Platform::Gemini => "gemini-1.0-pro",
        }
    }
}

impl Default for Platform {
    fn default() -> Self {
        Platform::Claude
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Language {
    Japanese,
    English,
    Chinese,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Japanese => "Japanese",
            Language::English => "English",
            Language::Chinese => "Chinese",
        }
    }

    pub fn system_prompt(&self) -> &'static str {
        match self {
            Language::Japanese => "あなたは優れたコミットメッセージを作成するエキスパートです。以下のGitの差分に基づいて、以下のフォーマットに従ったコンパクトで明確なコミットメッセージを生成してください。

フォーマット:
- 1行目: 変更の要約（50文字以内が望ましい）
- 2行目: 空行
- 3行目以降: 必要に応じて変更の詳細な説明（各行72文字以内が望ましい）

良いコミットメッセージの特徴:
1. 簡潔で明確
2. 何が変更されたかではなく「なぜ」変更されたかを説明
3. 関連する課題やバグ修正への参照を含める

コミットメッセージは日本語で生成してください。",
            Language::English => "You are an expert at creating commit messages. Based on the following Git diff, generate a compact and clear commit message following the format below.

Format:
- Line 1: Summary of the change (preferably under 50 characters)
- Line 2: Blank line
- Line 3+: Detailed explanation of the change if necessary (each line preferably under 72 characters)

Characteristics of a good commit message:
1. Concise and clear
2. Explains 'why' the change was made, not just what was changed
3. Includes references to related issues or bug fixes

Please generate the commit message in English.",
            Language::Chinese => "You are an expert at creating commit messages. Based on the following Git diff, generate a compact and clear commit message following the format below.

Format:
- Line 1: Summary of the change (preferably under 50 characters)
- Line 2: Blank line
- Line 3+: Detailed explanation of the change if necessary (each line preferably under 72 characters)

Please generate the commit message in Chinese.",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Japanese
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKeys {
    pub claude: Option<String>,
    pub openai: Option<String>,
    pub gemini: Option<String>,
}

impl ApiKeys {
    pub fn new() -> Self {
        Self {
            claude: None,
            openai: None,
            gemini: None,
        }
    }

    pub fn get_key(&self, platform: Platform) -> Option<String> {
        match platform {
            Platform::Claude => self.claude.clone(),
            Platform::OpenAI => self.openai.clone(),
            Platform::Gemini => self.gemini.clone(),
        }
    }

    pub fn set_key(&mut self, platform: Platform, key: String) {
        match platform {
            Platform::Claude => self.claude = Some(key),
            Platform::OpenAI => self.openai = Some(key),
            Platform::Gemini => self.gemini = Some(key),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_keys: ApiKeys,
    pub language: Language,
    pub platform: Platform,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_keys: ApiKeys::new(),
            language: Language::default(),
            platform: Platform::default(),
        }
    }

    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;

        if !config_path.exists() {
            return Ok(Config::new());
        }

        let mut file = File::open(&config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config = serde_json::from_str(&contents).unwrap_or_else(|_| Config::new());
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(&self)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&config_path)?;

        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn get_api_key(&self) -> Result<String> {
        // 環境変数から取得を試みる
        if let Ok(key) = std::env::var(self.platform.env_var_name()) {
            return Ok(key);
        }

        // 設定ファイルから取得
        if let Some(key) = self.api_keys.get_key(self.platform) {
            return Ok(key);
        }

        // APIキーが見つからない場合はエラー
        Err(anyhow!(
            "{} is not set. Please set it with 'ai_commit_cli config --api'",
            self.platform.env_var_name()
        ))
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("Could not find home directory"))?;
    Ok(home
        .join(".config")
        .join("ai_commit_cli")
        .join("config.json"))
}

// 対話式の選択機能 - 言語
pub fn select_language() -> Result<Language> {
    println!("Select language for commit messages:");
    println!("JA. Japanese (日本語)");
    println!("EN. English");
    println!("CN. Chinese");
    print!("Enter your choice (JA/EN/CN): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "JA" => Ok(Language::Japanese),
        "EN" => Ok(Language::English),
        "CN" => Ok(Language::Chinese),
        _ => {
            println!("Invalid choice. Using default (Japanese).");
            Ok(Language::Japanese)
        }
    }
}

// 対話式の選択機能 - プラットフォーム
pub fn select_platform() -> Result<Platform> {
    println!("Select AI platform:");
    println!("1. Claude (Anthropic)");
    println!("2. GPT-4 (OpenAI)");
    println!("3. Gemini (Google)");
    print!("Enter your choice (1-3): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "1" => Ok(Platform::Claude),
        "2" => Ok(Platform::OpenAI),
        "3" => Ok(Platform::Gemini),
        _ => {
            println!("Invalid choice. Using default (Claude).");
            Ok(Platform::Claude)
        }
    }
}

// 対話式の入力機能 - APIキー
pub fn input_api_key(platform: Platform) -> Result<String> {
    println!("Enter your {} API key:", platform.as_str());
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let api_key = input.trim().to_string();
    if api_key.is_empty() {
        return Err(anyhow!("API key cannot be empty"));
    }

    Ok(api_key)
}

pub async fn handle_config_command(api: &bool, show: &bool, language: &bool) -> Result<()> {
    let mut config = Config::load()?;

    if *api {
        // プラットフォームを選択
        let platform = select_platform()?;

        // APIキーを入力
        let api_key = input_api_key(platform)?;

        // 設定を保存
        config.api_keys.set_key(platform, api_key);
        config.platform = platform; // デフォルトのプラットフォームも更新
        config.save()?;

        println!("{} API key has been set successfully.", platform.as_str());
    }

    if *language {
        let selected_language = select_language()?;
        config.language = selected_language;
        config.save()?;
        println!("Language has been set to: {}", selected_language.as_str());
    }

    if *show || (!*api && !*language) {
        println!("Current configuration:");
        println!("Active platform: {}", config.platform.as_str());

        // 各プラットフォームのAPIキー状態を表示
        println!("API Keys:");

        // Claude
        if let Some(key) = &config.api_keys.claude {
            let masked_key = if key.len() > 8 {
                format!("{}********", &key[0..8])
            } else {
                "********".to_string()
            };
            println!("  Claude API Key: {}", masked_key);
        } else {
            println!("  Claude API Key: not set");
        }

        // OpenAI
        if let Some(key) = &config.api_keys.openai {
            let masked_key = if key.len() > 8 {
                format!("{}********", &key[0..8])
            } else {
                "********".to_string()
            };
            println!("  OpenAI API Key: {}", masked_key);
        } else {
            println!("  OpenAI API Key: not set");
        }

        // Gemini
        if let Some(key) = &config.api_keys.gemini {
            let masked_key = if key.len() > 8 {
                format!("{}********", &key[0..8])
            } else {
                "********".to_string()
            };
            println!("  Gemini API Key: {}", masked_key);
        } else {
            println!("  Gemini API Key: not set");
        }

        println!("Language: {}", config.language.as_str());
    }

    Ok(())
}
