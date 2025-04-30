use crate::language::Language;
use anyhow::{Result, anyhow};
use promptuity::{
    Promptuity, Term,
    prompts::{Select, SelectOption},
    themes::FancyTheme,
};
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
    pub custom_prompt: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_keys: ApiKeys::new(),
            language: Language::default(),
            platform: Platform::default(),
            custom_prompt: None,
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

// 対話式の選択機能 - プラットフォーム
pub fn select_platform() -> Result<Platform> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    let options = vec![
        (
            format!("Claude (Anthropic): {}", Platform::Claude.model_name()),
            Platform::Claude,
        ),
        (
            format!("GPT-4 (OpenAI): {}", Platform::OpenAI.model_name()),
            Platform::OpenAI,
        ),
        (
            format!("Gemini (Google): {}", Platform::Gemini.model_name()),
            Platform::Gemini,
        ),
    ];

    let select_options: Vec<SelectOption<String>> = options
        .iter()
        .map(|(label, _)| SelectOption::new(label.to_string(), label.to_string()))
        .collect();

    let mut select = Select::new("Select AI platform for commit messages", select_options);

    p.begin()?;
    let selected = p.prompt(&mut select)?;
    p.finish()?;

    // Find the matching platform based on the selected label
    let selected_platform = options
        .iter()
        .find(|(label, _)| label == &selected)
        .map(|(_, platform)| *platform)
        .unwrap_or(Platform::default());

    Ok(selected_platform)
}

pub fn input_api_key(platform: Platform) -> Result<String> {
    print!("Enter {} API key: ", platform.as_str());
    io::stdout().flush()?;

    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    Ok(api_key.trim().to_string())
}

pub async fn handle_config_command(
    api: &bool,
    show: &bool,
    language: &bool,
    prompt: &bool,
) -> Result<()> {
    let mut config = Config::load()?;

    if *show {
        println!("Current configuration:");
        println!("Language: {}", config.language.as_str());
        println!("Platform: {}", config.platform.as_str());
        println!(
            "Claude API key: {}",
            if config.api_keys.claude.is_some() {
                "Set"
            } else {
                "Not set"
            }
        );
        println!(
            "OpenAI API key: {}",
            if config.api_keys.openai.is_some() {
                "Set"
            } else {
                "Not set"
            }
        );
        println!(
            "Gemini API key: {}",
            if config.api_keys.gemini.is_some() {
                "Set"
            } else {
                "Not set"
            }
        );
        println!(
            "Custom prompt: {}",
            if let Some(_prompt) = &config.custom_prompt {
                "Set"
            } else {
                "Not set (using default)"
            }
        );
        return Ok(());
    }

    if *language {
        // 言語選択
        config.language = crate::language::select_language()?;
        println!("Language set to: {}", config.language.as_str());
        config.save()?;
    }

    if *api {
        // プラットフォーム選択
        config.platform = select_platform()?;
        println!("Platform set to: {}", config.platform.as_str());

        // APIキー入力
        let api_key = input_api_key(config.platform)?;
        config.api_keys.set_key(config.platform, api_key);
        println!("{} API key set successfully", config.platform.as_str());
        config.save()?;
    }

    if *prompt {
        // カスタムプロンプト入力
        let custom_prompt = input_custom_prompt()?;
        if custom_prompt.trim().is_empty() {
            config.custom_prompt = None;
            println!("Custom prompt cleared. Using default prompt for the selected language.");
        } else {
            config.custom_prompt = Some(custom_prompt);
            println!("Custom prompt set successfully");
        }
        config.save()?;
    }

    Ok(())
}

pub fn input_custom_prompt() -> Result<String> {
    println!("Custom prompt editor will open. Press Ctrl+S to save and Esc to exit.");
    println!("Write specific instructions for generating commit messages.");
    println!(
        "For example: \"You are an expert at creating concise and informative commit messages.\""
    );
    println!("\nPress Enter to continue...");

    // Enterキー入力待ち
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // 現在のカスタムプロンプトを取得（存在する場合）
    let config = Config::load()?;
    let initial_content = config.custom_prompt.unwrap_or_default();

    // エディタを起動
    let mut editor = crate::editor::Editor::with_content(&initial_content);
    let result = editor.run()?;

    // 末尾の改行を削除
    let result = result.trim_end().to_string();

    Ok(result)
}
