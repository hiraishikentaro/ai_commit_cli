use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::process::Command;
use tokio;

mod api;
mod config;
use config::Config;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate commit messages using AI")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(
        short,
        long,
        help = "Use the generated message to commit automatically"
    )]
    commit: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Set or get API configuration
    Config {
        /// Set API Key interactively
        #[arg(long, help = "Set API key interactively")]
        api: bool,

        /// Show current configuration
        #[arg(long, help = "Show current configuration")]
        show: bool,

        /// Set language for commit messages
        #[arg(long, help = "Set language for commit messages (interactive)")]
        language: bool,
    },
}

async fn get_staged_diff() -> Result<String> {
    let output = Command::new("git").args(["diff", "--staged"]).output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to get git diff: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8(output.stdout)?)
}

async fn generate_commit_message(diff: &str) -> Result<String> {
    // 設定を読み込み
    let config = Config::load()?;
    let api_key = config.get_api_key()?;
    let language = config.language;
    let platform = config.platform;

    // システムプロンプトと言語に応じたユーザープロンプトを取得
    let system_prompt = language.system_prompt();
    let user_prompt = match language {
        config::Language::Japanese => format!(
            "以下のGit差分に基づいてコミットメッセージを生成してください：\n\n```\n{}\n```",
            diff
        ),
        config::Language::English => format!(
            "Generate a commit message based on the following Git diff:\n\n```\n{}\n```",
            diff
        ),
        config::Language::Chinese => format!("根据以下Git差异生成提交消息：\n\n```\n{}\n```", diff),
    };

    // APIモジュールを使用してコミットメッセージを生成
    api::generate_commit_message(
        platform,
        &api_key,
        platform.model_name(),
        system_prompt,
        &user_prompt,
    )
    .await
}

async fn commit_with_message(message: &str) -> Result<()> {
    let commit_output = Command::new("git")
        .args(["commit", "-m", message])
        .output()?;

    if !commit_output.status.success() {
        return Err(anyhow!(
            "Failed to commit: {}",
            String::from_utf8_lossy(&commit_output.stderr)
        ));
    }

    println!("Committed successfully!");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // .envファイルから環境変数を読み込む
    dotenv().ok();

    // コマンドライン引数の解析
    let args = Args::parse();

    // サブコマンドの処理
    if let Some(command) = &args.command {
        match command {
            Commands::Config {
                api,
                show,
                language,
            } => {
                return config::handle_config_command(api, show, language).await;
            }
        }
    }

    // ステージされた差分を取得
    let diff = get_staged_diff().await?;

    if diff.is_empty() {
        println!("No staged changes found.");
        return Ok(());
    }

    // 設定を読み込み、使用するAIプラットフォームを表示
    let config = Config::load()?;
    println!(
        "Generating commit message using {}...",
        config.platform.as_str()
    );

    // コミットメッセージの生成
    let commit_message = generate_commit_message(&diff).await?;

    println!("\nGenerated commit message:\n{}", commit_message);

    // 自動コミットオプションが有効な場合
    if args.commit {
        println!("\nCommitting with the generated message...");
        commit_with_message(&commit_message).await?;
    } else {
        println!(
            "\nTo use this message for commit, run: git commit -m \"{}\"",
            commit_message.replace("\"", "\\\"")
        );
    }

    Ok(())
}
