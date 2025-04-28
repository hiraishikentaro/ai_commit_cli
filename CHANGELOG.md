# Changelog

All notable changes to the AI Commit CLI will be documented in this file.

## [0.0.1] - 2024-06-17

### Features

- Initial release of the AI Commit CLI tool
- Support for multiple AI platforms:
  - Claude (Anthropic)
  - GPT-4 (OpenAI)
  - Gemini (Google)
- Configuration management for API keys
- Language options for commit messages:
  - Japanese (default)
  - English
  - Chinese
- Command-line functionality:
  - Generate commit messages based on staged Git changes
  - Automatic commit with generated messages (`--commit` or `-c` flag)
  - Interactive configuration setup
- API key management through environment variables or configuration file
- Detailed error handling and user feedback
