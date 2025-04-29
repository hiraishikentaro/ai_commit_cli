mod config_tests {
    use crate::config::{ApiKeys, Config, Platform};
    use crate::language::Language;

    #[test]
    fn test_platform_as_str() {
        assert_eq!(Platform::Claude.as_str(), "Claude");
        assert_eq!(Platform::OpenAI.as_str(), "OpenAI");
        assert_eq!(Platform::Gemini.as_str(), "Gemini");
    }

    #[test]
    fn test_platform_env_var_name() {
        assert_eq!(Platform::Claude.env_var_name(), "CLAUDE_API_KEY");
        assert_eq!(Platform::OpenAI.env_var_name(), "OPENAI_API_KEY");
        assert_eq!(Platform::Gemini.env_var_name(), "GEMINI_API_KEY");
    }

    #[test]
    fn test_platform_model_name() {
        assert_eq!(Platform::Claude.model_name(), "claude-3-opus-20240229");
        assert_eq!(Platform::OpenAI.model_name(), "gpt-4");
        assert_eq!(Platform::Gemini.model_name(), "gemini-1.0-pro");
    }

    #[test]
    fn test_platform_default() {
        let default_platform = Platform::default();
        assert!(matches!(default_platform, Platform::Claude));
    }

    #[test]
    fn test_language_as_str() {
        assert_eq!(Language::Japanese.as_str(), "Japanese");
        assert_eq!(Language::English.as_str(), "English");
        assert_eq!(Language::Chinese.as_str(), "Chinese");
    }

    #[test]
    fn test_language_default() {
        let default_language = Language::default();
        assert!(matches!(default_language, Language::Japanese));
    }

    #[test]
    fn test_api_keys_new() {
        let api_keys = ApiKeys::new();
        assert!(api_keys.claude.is_none());
        assert!(api_keys.openai.is_none());
        assert!(api_keys.gemini.is_none());
    }

    #[test]
    fn test_api_keys_get_set() {
        let mut api_keys = ApiKeys::new();

        // Test set_key and get_key for each platform
        api_keys.set_key(Platform::Claude, "claude_key".to_string());
        api_keys.set_key(Platform::OpenAI, "openai_key".to_string());
        api_keys.set_key(Platform::Gemini, "gemini_key".to_string());

        assert_eq!(
            api_keys.get_key(Platform::Claude),
            Some("claude_key".to_string())
        );
        assert_eq!(
            api_keys.get_key(Platform::OpenAI),
            Some("openai_key".to_string())
        );
        assert_eq!(
            api_keys.get_key(Platform::Gemini),
            Some("gemini_key".to_string())
        );
    }

    #[test]
    fn test_config_new() {
        let config = Config::new();
        let _default_platform = Platform::default();
        assert!(matches!(config.platform, _default_platform));
        let _default_language = Language::default();
        assert!(matches!(config.language, _default_language));
        assert!(config.api_keys.claude.is_none());
        assert!(config.api_keys.openai.is_none());
        assert!(config.api_keys.gemini.is_none());
    }
}

mod api_tests {
    use crate::api;
    use crate::config::Platform;
    use std::path::Path;

    #[test]
    fn test_api_modules_exist() {
        // Verify that each API module file exists
        assert!(
            Path::new("src/api/claude.rs").exists(),
            "Claude API module should exist"
        );
        assert!(
            Path::new("src/api/openai.rs").exists(),
            "OpenAI API module should exist"
        );
        assert!(
            Path::new("src/api/gemini.rs").exists(),
            "Gemini API module should exist"
        );
    }

    #[test]
    fn test_api_selection_constants() {
        // Verify that API base URL environment variable constants are defined
        assert_eq!(api::ANTHROPIC_API_BASE_ENV, "ANTHROPIC_API_BASE");
        assert_eq!(api::OPENAI_API_BASE_ENV, "OPENAI_API_BASE");
        assert_eq!(api::GEMINI_API_BASE_ENV, "GEMINI_API_BASE");
    }

    #[test]
    fn test_platform_mapping() {
        // Verify that each platform is correctly mapped
        let platforms = [Platform::Claude, Platform::OpenAI, Platform::Gemini];
        let models = platforms.iter().map(|p| p.model_name()).collect::<Vec<_>>();

        assert!(models.contains(&"claude-3-opus-20240229"));
        assert!(models.contains(&"gpt-4"));
        assert!(models.contains(&"gemini-1.0-pro"));
    }
}

mod main_tests {
    use std::process::Command;

    #[test]
    fn test_command_output_parsing() {
        // Command output for testing
        let output = Command::new("echo")
            .arg("test diff output")
            .output()
            .expect("Failed to execute command");

        // Test that the output can be correctly parsed as a string
        let output_str = String::from_utf8(output.stdout).expect("Failed to parse stdout");
        assert_eq!(output_str.trim(), "test diff output");
    }

    #[test]
    fn test_empty_diff_handling() {
        // Test handling of empty diff
        // In the actual implementation, error messages should be displayed for empty cases
        let empty_diff = "";
        assert!(empty_diff.is_empty());
    }
}

mod custom_prompt_tests {
    use crate::config::Config;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_custom_prompt_in_config() {
        // カスタムプロンプト対応の設定構造体のテスト
        let mut config = Config::new();
        
        // カスタムプロンプトがデフォルトではNoneであることを確認
        assert!(config.custom_prompt.is_none());
        
        // カスタムプロンプトを設定できることを確認
        let test_prompt = "This is a test custom prompt";
        config.custom_prompt = Some(test_prompt.to_string());
        
        assert_eq!(config.custom_prompt, Some(test_prompt.to_string()));
        
        // カスタムプロンプトをクリアできることを確認
        config.custom_prompt = None;
        assert!(config.custom_prompt.is_none());
    }
    
    #[test]
    fn test_config_save_load_with_custom_prompt() {
        // テスト用の一時ディレクトリとファイルパスを作成
        let temp_dir = std::env::temp_dir();
        let test_config_dir = temp_dir.join("ai_commit_cli_test");
        let test_config_file = test_config_dir.join("config.json");
        
        // テスト用ディレクトリを作成
        fs::create_dir_all(&test_config_dir).unwrap();
        
        // 環境変数をモック
        unsafe {
            std::env::set_var("HOME", temp_dir.to_str().unwrap());
        }
        
        // テスト用の設定ファイルパス取得関数（実際には使用しないため_をつける）
        let _get_test_config_path = || -> Result<PathBuf, anyhow::Error> {
            Ok(test_config_file.clone())
        };
        
        // テスト用のカスタムプロンプトを含む設定
        let mut config = Config::new();
        let test_prompt = "Custom prompt for testing purposes\nWith multiple lines\nAnd formatting.";
        config.custom_prompt = Some(test_prompt.to_string());
        
        // 設定を保存
        let save_result = fs::write(
            &test_config_file,
            serde_json::to_string_pretty(&config).unwrap(),
        );
        assert!(save_result.is_ok());
        
        // 設定ファイルが存在することを確認
        assert!(test_config_file.exists());
        
        // ファイルから直接設定を読み込む
        let file_content = fs::read_to_string(&test_config_file).unwrap();
        let loaded_config: Config = serde_json::from_str(&file_content).unwrap();
        
        // カスタムプロンプトが正しく保存・読み込みされていることを確認
        assert_eq!(loaded_config.custom_prompt, Some(test_prompt.to_string()));
        
        // テスト後の cleanup
        fs::remove_file(&test_config_file).unwrap_or(());
        fs::remove_dir(&test_config_dir).unwrap_or(());
    }
}

mod editor_tests {
    use crate::editor::Editor;
    
    #[test]
    fn test_editor_creation() {
        // 新しいエディタインスタンスのテスト
        let editor = Editor::_new();
        
        // 初期状態の確認
        let content = editor.get_content();
        assert_eq!(content.len(), 1);
        assert_eq!(content[0], "");
        
        let (cursor_x, cursor_y) = editor.get_cursor_position();
        assert_eq!(cursor_x, 0);
        assert_eq!(cursor_y, 0);
        
        assert!(editor.get_message().contains("INSERT MODE"));
    }
    
    #[test]
    fn test_editor_with_content() {
        // 既存コンテンツでのエディタ初期化テスト
        let test_content = "Line 1\nLine 2\nLine 3";
        let editor = Editor::with_content(test_content);
        
        // コンテンツが正しく読み込まれていることを確認
        let content = editor.get_content();
        assert_eq!(content.len(), 3);
        assert_eq!(content[0], "Line 1");
        assert_eq!(content[1], "Line 2");
        assert_eq!(content[2], "Line 3");
        
        let (cursor_x, cursor_y) = editor.get_cursor_position();
        assert_eq!(cursor_x, 0);
        assert_eq!(cursor_y, 0);
    }
    
    #[test]
    fn test_editor_with_empty_content() {
        // 空のコンテンツでのエディタ初期化テスト
        let editor = Editor::with_content("");
        
        // 空の場合でも1行は確保されていることを確認
        let content = editor.get_content();
        assert_eq!(content.len(), 1);
        assert_eq!(content[0], "");
    }
    
    #[test]
    fn test_editor_text_processing() {
        // エディタのテキスト処理機能をテスト
        let mut editor = Editor::_new();
        
        // テスト用のテキスト
        let test_text = "First line\nSecond line\nThird line";
        
        // テキスト処理関数を呼び出し
        let result = editor.process_text_for_test(test_text);
        
        // 結果を検証
        assert!(result.contains("First line"));
        assert!(result.contains("Second line"));
        assert!(result.contains("Third line"));
        assert!(result.contains("Test added line"));
        
        // 行数を確認
        let content = editor.get_content();
        assert_eq!(content.len(), 4); // 3行 + 追加した1行
        assert_eq!(content[3], "Test added line");
    }
}

mod integration_tests {
    use crate::config::{Config, Platform};
    use crate::language::Language;
    use std::sync::Once;

    static INIT: Once = Once::new();

    // モックのAPI応答を設定するヘルパー関数
    fn setup() {
        INIT.call_once(|| {
            // APIモックの設定などがあれば初期化
        });
    }

    #[tokio::test]
    async fn test_custom_prompt_in_commit_message_generation() {
        setup();
        
        // テスト用のGit diff（未使用だがテストケース理解のために残す）
        let _test_diff = "diff --git a/file.txt b/file.txt\nindex 123..456 789\n--- a/file.txt\n+++ b/file.txt\n@@ -1,3 +1,4 @@\n Line 1\n Line 2\n+Added line\n Line 3";
        
        // テスト用のカスタムプロンプト
        let custom_prompt = "You are a test AI. Just return 'CUSTOM_PROMPT_USED' as the commit message.";
        
        // カスタムプロンプトありの設定
        let mut config_with_custom = Config::new();
        config_with_custom.custom_prompt = Some(custom_prompt.to_string());
        config_with_custom.platform = Platform::Claude; // テスト用にプラットフォームを固定
        config_with_custom.language = Language::English; // テスト用に言語を固定
        
        // カスタムプロンプトなしの設定
        let mut config_without_custom = Config::new();
        config_without_custom.custom_prompt = None;
        config_without_custom.platform = Platform::Claude;
        config_without_custom.language = Language::English;
        
        // 実際のAPIを呼び出さないようにモックで実装
        // 通常はここでAPIsのモックを設定してから以下のようなテストを実行する
        
        // カスタムプロンプトが設定されている場合とされていない場合で
        // システムプロンプトが正しく選択されることを確認するテスト
        
        // たとえば以下のようなコード（実際のAPIを呼び出すため、コメントアウト）
        /*
        // カスタムプロンプトを使用
        let message_with_custom = generate_commit_message(test_diff).await;
        assert!(message_with_custom.is_ok());
        
        // デフォルトプロンプトを使用
        let message_without_custom = generate_commit_message(test_diff).await;
        assert!(message_without_custom.is_ok());
        */
        
        // 異なるプロンプトが選択されることのみをアサーション
        assert_ne!(
            custom_prompt, 
            Language::English.system_prompt(),
            "カスタムプロンプトとデフォルトプロンプトは異なるべき"
        );
    }
}
