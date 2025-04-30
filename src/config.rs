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

    pub fn default_model_name(&self) -> &'static str {
        match self {
            Platform::Claude => "claude-3-opus-20240229",
            Platform::OpenAI => "gpt-4",
            Platform::Gemini => "gemini-1.0-pro",
        }
    }

    pub fn get_models(&self) -> Vec<(&'static str, &'static str)> {
        match self {
            Platform::Claude => vec![
                ("Claude 3.7 Sonnet", "claude-3-7-sonnet-20250219"),
                ("Claude 3.5 Sonnet	", "claude-3-5-sonnet-20240620"),
                ("Claude 3.5 Haiku", "claude-3-5-haiku-20241022"),
            ],
            Platform::OpenAI => vec![
                ("o4-mini", "o4-mini-2025-04-16"),
                ("GPT-4.1", "gpt-4.1-2025-04-14"),
                ("o3-mini", "o3-mini-2025-01-31"),
                ("o3", "o3-2025-04-16"),
            ],
            Platform::Gemini => vec![
                ("Gemini 2.5 Flash", "gemini-2.5-flash-preview-04-17"),
                ("Gemini 2.5 Pro", "gemini-2.5-pro-preview-03-25"),
                ("Gemini 2.0 Flash", "models/gemini-2.0-flash"),
            ],
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
    pub selected_model: Option<String>,
    pub custom_prompt: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_keys: ApiKeys::new(),
            language: Language::default(),
            platform: Platform::default(),
            selected_model: None,
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

    pub fn get_model_name(&self) -> String {
        self.selected_model
            .clone()
            .unwrap_or_else(|| self.platform.default_model_name().to_string())
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("Could not find home directory"))?;
    Ok(home
        .join(".config")
        .join("ai_commit_cli")
        .join("config.json"))
}

// 対話式の選択機能 - プラットフォームとモデル
pub fn select_platform_and_model() -> Result<(Platform, String)> {
    // まずプラットフォームを選択
    let platform = select_platform_only()?;

    // 次にモデルを選択
    let model = select_model(platform)?;

    Ok((platform, model))
}

// プラットフォームのみ選択する関数（内部用）
fn select_platform_only() -> Result<Platform> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    let options = vec![
        ("Claude (Anthropic)", Platform::Claude),
        ("OpenAI (GPT)", Platform::OpenAI),
        ("Gemini (Google)", Platform::Gemini),
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

// プラットフォームに対応するモデルを選択する関数
pub fn select_model(platform: Platform) -> Result<String> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    let models = platform.get_models();
    let select_options: Vec<SelectOption<String>> = models
        .iter()
        .map(|(label, id)| SelectOption::new(label.to_string(), id.to_string()))
        .collect();

    let mut select = Select::new(
        format!("Select {} model", platform.as_str()),
        select_options,
    );

    p.begin()?;
    let selected_model_id = p.prompt(&mut select)?;
    p.finish()?;

    Ok(selected_model_id)
}

pub fn input_api_key(platform: Platform) -> Result<String> {
    print!("Enter {} API key: ", platform.as_str());
    io::stdout().flush()?;

    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    Ok(api_key.trim().to_string())
}

// 再帰を避けるために、引数からコピーして新しい関数として実装
async fn do_config(api: bool, show: bool, language: bool, prompt: bool) -> Result<()> {
    let mut config = Config::load()?;

    if show {
        println!("Current configuration:");
        println!("Language: {}", config.language.as_str());
        println!("Platform: {}", config.platform.as_str());
        println!("Model: {}", config.get_model_name());
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
            if config.custom_prompt.is_some() {
                "Set"
            } else {
                "Not set"
            }
        );

        return Ok(());
    }

    if api {
        // プラットフォームとモデルを選択
        let (platform, model) = select_platform_and_model()?;

        // プラットフォームとモデルを設定に保存
        config.platform = platform;
        config.selected_model = Some(model);

        // APIキーを入力
        let api_key = input_api_key(platform)?;
        config.api_keys.set_key(platform, api_key);

        // 設定を保存
        config.save()?;
        println!("API configuration and model saved successfully.");
        return Ok(());
    }

    if language {
        config.language = crate::language::select_language()?;
        config.save()?;
        println!("Language set to: {}", config.language.as_str());
    }

    if prompt {
        config.custom_prompt = Some(input_custom_prompt()?);
        config.save()?;
        println!("Custom prompt saved successfully.");
    }

    // 何も指定されていない場合は設定メニューを表示
    if !(api || show || language || prompt) {
        // 非同期再帰呼び出しをBoxでラップ
        return Box::pin(do_config(true, false, false, false)).await;
    }

    Ok(())
}

pub async fn handle_config_command(
    api: &bool,
    show: &bool,
    language: &bool,
    prompt: &bool,
) -> Result<()> {
    do_config(*api, *show, *language, *prompt).await
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
