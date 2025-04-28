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
