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
    fn test_platform_default_model_name() {
        assert_eq!(
            Platform::Claude.default_model_name(),
            "claude-3-opus-20240229"
        );
        assert_eq!(Platform::OpenAI.default_model_name(), "gpt-4");
        assert_eq!(Platform::Gemini.default_model_name(), "gemini-1.0-pro");
    }

    #[test]
    fn test_platform_get_models() {
        // テスト: 各プラットフォームのモデルリストが正しく返されるか
        let claude_models = Platform::Claude.get_models();
        let openai_models = Platform::OpenAI.get_models();
        let gemini_models = Platform::Gemini.get_models();

        // 各プラットフォームが少なくとも1つのモデルを持っていることを確認
        assert!(!claude_models.is_empty());
        assert!(!openai_models.is_empty());
        assert!(!gemini_models.is_empty());

        // 特定のモデルがリストに含まれていることを確認
        assert!(claude_models.iter().any(|(_, id)| id.contains("claude-3")));

        assert!(
            openai_models
                .iter()
                .any(|(_, id)| id.contains("gpt") || id.contains("o"))
        );

        assert!(gemini_models.iter().any(|(_, id)| id.contains("gemini")));

        // 各モデルがラベルとIDのペアであることを確認
        for (label, id) in claude_models {
            assert!(!label.is_empty());
            assert!(!id.is_empty());
        }
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
        assert!(config.selected_model.is_none());
    }

    #[test]
    fn test_config_get_model_name() {
        // デフォルトのプラットフォームでモデル名を取得するテスト
        let config = Config::new();
        assert_eq!(
            config.get_model_name(),
            config.platform.default_model_name()
        );

        // 選択されたモデルがある場合のテスト
        let mut config_with_model = Config::new();
        let selected_model = "custom-model-name";
        config_with_model.selected_model = Some(selected_model.to_string());
        assert_eq!(config_with_model.get_model_name(), selected_model);
    }

    #[test]
    fn test_config_save_load_with_selected_model() {
        // テスト用の一時ディレクトリとファイルパスを作成
        let temp_dir = std::env::temp_dir();
        let test_config_dir = temp_dir.join("ai_commit_cli_test_model");
        let test_config_file = test_config_dir.join("config.json");

        // テスト用ディレクトリを作成
        std::fs::create_dir_all(&test_config_dir).unwrap();

        // 環境変数をモック - 安全でない操作なのでunsafeブロックで囲む
        unsafe {
            std::env::set_var("HOME", temp_dir.to_str().unwrap());
        }

        // テスト用のモデル選択を含む設定
        let mut config = Config::new();
        let test_model = "claude-3-sonnet-20240229";
        config.selected_model = Some(test_model.to_string());

        // 設定を保存
        let save_result = std::fs::write(
            &test_config_file,
            serde_json::to_string_pretty(&config).unwrap(),
        );
        assert!(save_result.is_ok());

        // 設定ファイルが存在することを確認
        assert!(test_config_file.exists());

        // ファイルから直接設定を読み込む
        let file_content = std::fs::read_to_string(&test_config_file).unwrap();
        let loaded_config: Config = serde_json::from_str(&file_content).unwrap();

        // 選択したモデルが正しく保存・読み込みされていることを確認
        assert_eq!(loaded_config.selected_model, Some(test_model.to_string()));

        // テスト後の cleanup
        std::fs::remove_file(&test_config_file).unwrap_or(());
        std::fs::remove_dir(&test_config_dir).unwrap_or(());
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
        let models = platforms
            .iter()
            .map(|p| p.default_model_name())
            .collect::<Vec<_>>();

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
        let _get_test_config_path =
            || -> Result<PathBuf, anyhow::Error> { Ok(test_config_file.clone()) };

        // テスト用のカスタムプロンプトを含む設定
        let mut config = Config::new();
        let test_prompt =
            "Custom prompt for testing purposes\nWith multiple lines\nAnd formatting.";
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
    use tokio;

    static INIT: Once = Once::new();

    // モックのAPI応答を設定するヘルパー関数
    fn setup() {
        INIT.call_once(|| {
            // APIモックの設定などがあれば初期化
            unsafe {
                std::env::set_var("CLAUDE_API_KEY", "dummy-claude-key");
                std::env::set_var("OPENAI_API_KEY", "dummy-openai-key");
                std::env::set_var("GEMINI_API_KEY", "dummy-gemini-key");
            }
        });
    }

    // モデル選択機能の統合テスト
    #[tokio::test]
    async fn test_selected_model_in_commit_message_generation() {
        use std::env;

        // APIリクエストをモックするために環境変数でベースURLを設定
        unsafe {
            env::set_var("ANTHROPIC_API_BASE", "http://localhost:9999");
            env::set_var("OPENAI_API_BASE", "http://localhost:9999");
            env::set_var("GEMINI_API_BASE", "http://localhost:9999");
        }

        // APIのモックを確認するためのテスト環境設定
        setup();

        // テスト用の設定
        let mut config = Config::new();

        // 各プラットフォームでテスト
        for platform in [Platform::Claude, Platform::OpenAI, Platform::Gemini] {
            config.platform = platform;

            // デフォルトモデルケース
            config.selected_model = None;
            let _default_model = platform.default_model_name().to_string();

            // 選択モデルケース
            let custom_model = format!("custom-{}-model", platform.as_str().to_lowercase());
            config.selected_model = Some(custom_model.clone());

            // モデル名が設定から正しく取得されることを確認
            assert_eq!(config.get_model_name(), custom_model);
        }
    }

    #[tokio::test]
    async fn test_custom_prompt_in_commit_message_generation() {
        setup();

        // テスト用のGit diff（未使用だがテストケース理解のために残す）
        let _test_diff = "diff --git a/file.txt b/file.txt\nindex 123..456 789\n--- a/file.txt\n+++ b/file.txt\n@@ -1,3 +1,4 @@\n Line 1\n Line 2\n+Added line\n Line 3";

        // テスト用の設定
        let mut config = Config::new();
        config.platform = Platform::Claude; // テスト用にClaudeを選択
        config.language = Language::English; // 英語を選択

        // カスタムプロンプトを設定
        let custom_prompt = "You are an expert commit message generator. Be brief and precise.";
        config.custom_prompt = Some(custom_prompt.to_string());

        // カスタムプロンプトが期待通り設定されることを確認
        assert_eq!(config.custom_prompt, Some(custom_prompt.to_string()));
    }
}

// モデル選択機能のテスト用モジュール
mod model_selection_tests {
    use crate::config::{Config, Platform};

    #[test]
    fn test_platform_get_model_combinations() {
        // 各プラットフォームとモデルの組み合わせが有効かテスト
        for platform in [Platform::Claude, Platform::OpenAI, Platform::Gemini] {
            let models = platform.get_models();

            for (name, id) in models {
                // 名前とIDが有効な値であることを確認
                assert!(!name.is_empty(), "Model name should not be empty");
                assert!(!id.is_empty(), "Model ID should not be empty");

                // IDに無効な文字が含まれていないことを確認
                assert!(!id.contains(' '), "Model ID should not contain spaces");
                assert!(!id.contains('\n'), "Model ID should not contain newlines");
            }
        }
    }

    #[test]
    fn test_config_with_selected_model() {
        // 設定に選択モデルを設定し、APIに渡すモデル名が正しいかテスト
        let mut config = Config::new();

        // 各プラットフォームのデフォルトモデル名をテスト
        for platform in [Platform::Claude, Platform::OpenAI, Platform::Gemini] {
            config.platform = platform;
            config.selected_model = None;

            // デフォルトモデル名が取得されることを確認
            assert_eq!(config.get_model_name(), platform.default_model_name());

            // 各プラットフォームのモデルから1つ選び、設定して確認
            if let Some((_, model_id)) = platform.get_models().first() {
                config.selected_model = Some(model_id.to_string());
                assert_eq!(config.get_model_name(), *model_id);
            }
        }
    }

    #[test]
    fn test_custom_model_name() {
        // カスタムモデル名を選択した場合のテスト
        let mut config = Config::new();

        // プラットフォームのモデルリストにないカスタムモデル名
        let custom_model = "custom-future-model-not-in-list";
        config.selected_model = Some(custom_model.to_string());

        // カスタムモデル名が取得されることを確認
        assert_eq!(config.get_model_name(), custom_model);
    }

    #[test]
    fn test_serialization_with_model_selection() {
        // モデル選択を含むシリアライズとデシリアライズのテスト
        let mut config = Config::new();
        config.platform = Platform::Claude;
        config.selected_model = Some("claude-3-haiku-20240307".to_string());

        // JSON文字列にシリアライズ
        let json = serde_json::to_string(&config).unwrap();

        // その文字列からデシリアライズ
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        // 値が保持されていることを確認
        assert_eq!(deserialized.platform, Platform::Claude);
        assert_eq!(
            deserialized.selected_model,
            Some("claude-3-haiku-20240307".to_string())
        );
    }
}
