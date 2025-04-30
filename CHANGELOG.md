# Changelog

All notable changes to the AI Commit CLI will be documented in this file.

## [0.0.3] - 2025-04-30

### Added

- AI model selection feature for each platform
- Custom prompt support with vim-like editor
  - New `--prompt` flag in the `config` subcommand
  - Multi-line prompt editing capability

### Changed

- Updated AI model names and IDs to reflect latest offerings
  - Added support for Gemini 1.5 Pro
  - Added support for GPT-4o-mini
  - Fixed model ID inconsistencies
- Improved test coverage for platform-model mapping

## [0.0.2] - 2025-04-28

### Added

- Interactive configuration selection using promptuity
- Homebrew installation support
  - Added Homebrew formula
  - Implemented GitHub Actions workflow for testing

## [0.0.1] - 2025-04-28

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
